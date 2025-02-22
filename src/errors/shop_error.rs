use axum::{body::Body, http, response::IntoResponse, BoxError, Json};
use thiserror::Error;

use std::{
    error::Error,
    net::{IpAddr, SocketAddr},
};

use crate::{
    helpers::{aws::HasAWSSdkConfig, dynamodb::HasDynamoDBConfiguration, sqs::HasSQSConfiguration},
    models::Ticket,
};

#[cfg(doc)]
use crate::models::{Barista, Waiter};

use super::{CoffeeMachineError, ErrorSchema};

/// For error handling of SQS SDK.
mod sqs {
    pub const DEFAULT_ERROR_MESSAGE: &str = "(No details provided)";

    pub use aws_sdk_sqs::types::error::*;
    pub use aws_sdk_sqs::Error;
}
/// For the error handling of DynamoDB SDK.
mod dynamodb {
    pub use aws_sdk_dynamodb::types::error::*;
    pub use aws_sdk_dynamodb::Error;
}

mod sts {
    pub use aws_sdk_sts::Error;

    // Trait for providing error metadata.
    pub(super) use aws_sdk_sts::error::ProvideErrorMetadata;
}
use sts::ProvideErrorMetadata;

#[derive(Error, Debug, strum::IntoStaticStr)]
pub enum CoffeeShopError {
    #[error("Invalid configuration for {field}: {message}")]
    InvalidConfiguration {
        field: &'static str,
        message: String,
    },

    #[error("{0:?} is not a valid multicast address.")]
    InvalidMulticastAddress(IpAddr),

    #[error("Received an invalid MulticastMessage from {addr}.")]
    InvalidMulticastMessage {
        data: Vec<u8>,
        addr: String,
        error: prost::DecodeError,
    },

    #[error("Header {key} is not acceptable: {message}")]
    InvalidHeader {
        key: http::header::HeaderName,
        message: String,
    },

    #[error("The request Method is not allowed for this endpoint.")]
    InvalidMethod,

    #[error("Endpoint {0} is not found on this server. Please consult the API documentation.")]
    InvalidRoute(http::Uri),

    // This needs to be reworked to be more specific.
    #[error("Invalid URL query options: {0}")]
    InvalidQueryOptions(String),

    #[error("Malformed JSON payload; could not be parsed: {0}")]
    MalformedJsonPayload(String),

    // This needs to be reworked to be more specific.
    #[error("Failed to parse the payload due to {kind}: {message}")]
    InvalidPayload { kind: &'static str, message: String },

    #[error("HTTP Host failed: {0}")]
    HTTPServerError(std::io::ErrorKind, std::io::Error),

    #[error("Failed to bind listener to socket address {1}: {0}")]
    ListenerCreationFailure(String, SocketAddr),

    #[error("Could not serialize the payload: {0}")]
    BinaryConversionError(#[from] Box<bincode::ErrorKind>),

    #[error("Could not compress/decompress the payload: {0}")]
    BinaryCompressionError(#[from] lzma::LzmaError),

    #[error("The payload is too large after compression: {0} bytes")]
    SizeLimitExceeded(usize),

    #[error("Temporary directory could not be created: {0}")]
    TempDirCreationFailure(String),

    #[error("Temporary file access failure at {path}: {reason}")]
    TempFileAccessFailure {
        path: std::path::PathBuf,
        reason: String,
    },

    #[error("The path for a temporary file is non-uniquely generated; this is improbable unless cleanup is not working. Please verify.")]
    NonUniqueTemporaryFile,

    #[error("A thread related system resource error had occurred: {0}")]
    ThreadResourceError(String),

    #[error("Failed to decode from Base64: {0}")]
    Base64DecodingError(#[from] base64::DecodeError),

    #[error("The requested payload is {0} bytes in size, exceeding the limit; try chunking the payload and retry the request.")]
    Base64EncodingOversize(usize),

    #[error("An IOError::{0} had occurred: {1}")]
    IOError(std::io::ErrorKind, std::io::Error),

    #[error("An IOError::{0} had occurred during multicast operations: {1}")]
    MulticastIOError(std::io::ErrorKind, std::io::Error),

    #[error("Timed out awaiting results after {0:?} seconds")]
    RetrieveTimeout(tokio::time::Duration),

    #[error("An error relating to AWS IAM credentials occurred: {0}")]
    AWSCredentialsError(String),

    #[error("AWS Configuration incomplete: {0}")]
    AWSConfigIncomplete(String),

    /// Generic DynamoDB Error that we fallback to, if we don't know the specific error.
    ///
    /// [`dynamodb::Error`] is non-exhaustive, so we need to handle the unknown errors.
    #[error("An error occurred while processing the request: {0}")]
    AWSDynamoDBResponseError(#[from] Box<dynamodb::Error>),

    #[error("The specified AWS DynamoDB Table does not exists. Please verify the table: {0}")]
    AWSDynamoDBTableDoesNotExist(String),

    #[error("DynamoDB item is found malformed: {0}")]
    AWSDynamoDBMalformedItem(String),

    #[error("DynamoDB item operation could not be performed due to {kind}: {message}")]
    AWSDynamoDBItemOperationError { kind: String, message: String },

    #[error("DynamoDB item already exists: {0}")]
    AWSDynamoDBDuplicateItem(String),

    #[error("AWS DynamoDB Provisioned Throughput Exceeded.")]
    AWSDynamoDBRateLimitExceeded,

    #[error("The AWS DynamoDB table has exceeded the item collection size limit.")]
    AWSDynamoDBCollectionOversize,

    #[error("The specified AWS SQS queue URL does not exists. Please verify the URL: {0}")]
    AWSQueueDoesNotExist(String),

    #[error("AWS SQS Rejected the message: {0}; please verify the payload and try again.")]
    AWSSQSInvalidMessage(String),

    #[error("AWS SQS Queue is empty after waiting for {0:?}.")]
    AWSSQSQueueEmpty(tokio::time::Duration),

    #[error("AWS SQS Queue is being purged; please wait a minute before trying again. This error should not be encountered in production.")]
    AWSSQSQueueBeingPurged,

    #[error("An error was reported by AWS SQS: {0}")]
    AWSSQSResponseError(#[from] Box<sqs::Error>),

    #[error("Message from AWS SQS had already been completed, and cannot be {0} again.")]
    AWSSQSStagedReceiptAlreadyCompleted(&'static str),

    #[error("AWS responded with unexpected data: {0}")]
    UnexpectedAWSResponse(String),

    /// Generic AWS SDK error.
    ///
    /// Use this as a last resort, as it is not specific to any SDK.
    #[error("AWS SDK Error: {0}")]
    AWSSdkError(String),

    #[error("Error during processing: {0}")]
    ProcessingError(#[from] CoffeeMachineError),

    #[error("Result is already set, cannot set again.")]
    ResultAlreadySet,

    #[error("The ticket {0} does not have a result. It could have been purged, or the ticket is invalid.")]
    ResultNotFound(Ticket),

    #[error("The ticket {0} was not found.")]
    TicketNotFound(Ticket),

    #[error("Upstream worker reported an error: {0:?}")]
    ErrorSchema(ErrorSchema),

    #[cfg(test)]
    #[error("A unit test failed unexpectedly: {0}")]
    UnitTestFailure(String),
}

impl CoffeeShopError {
    /// Convenient method to create a [`CoffeeShopError::IOError`] variant from [`std::io::Error`].
    pub fn from_io_error(error: std::io::Error) -> Self {
        if error.kind() == std::io::ErrorKind::AlreadyExists {
            Self::NonUniqueTemporaryFile
        } else {
            Self::IOError(error.kind(), error)
        }
    }

    /// Convenient method to create a [`CoffeeShopError::MulticastIOError`] variant from [`std::io::Error`].
    pub fn from_multicast_io_error(error: std::io::Error) -> Self {
        // We don't need to care about unique temporary files here.
        Self::MulticastIOError(error.kind(), error)
    }

    /// Convenient method to create a [`CoffeeShopError::HTTPServerError`] variant from [`std::io::Error`].
    pub fn from_server_io_error(error: std::io::Error) -> Self {
        // We don't need to care about unique temporary files here.
        Self::HTTPServerError(error.kind(), error)
    }

    /// Convenient method to map AWS STS [`sts::Error`] to [`CoffeeShopError`].
    pub fn from_aws_sts_error(error: sts::Error, config: &dyn HasAWSSdkConfig) -> Self {
        crate::error!(
            "An error occurred during AWS STS validation using config\n:{sdk_config:#?}",
            sdk_config = config.aws_config()
        );
        match error {
            sts::Error::ExpiredTokenException(_) =>
                Self::AWSCredentialsError("The AWS credentials have already expired at start up. If you are assuming an IAM role, please ensure that the role will last long enough for the lifetime of this instance.".to_owned())
            ,
            sts::Error::InvalidIdentityTokenException(_) =>
                Self::AWSCredentialsError("The AWS credentials are invalid at start up. If you are assuming an IAM role, please ensure that the role will last long enough for the lifetime of this instance.".to_owned())
            ,
            error => if let Some(code) = error.code() {
                // The AWS recommended way of handling `Unhandled` errors, but it does not seem
                // to be capturing anything.
                Self::AWSSdkError(format!("{code} during startup AWS credentials validation: {error:#?}"))
            } else if let Some(source) = error.source() {
                Self::AWSSdkError(format!("An error occurred during startup AWS credentials validation: {source:#?}"))
            } else {
                Self::AWSSdkError(format!("An unspecified error occurred during startup AWS credentials validation: {error:#?}"))
            },
        }
    }

    /// Convenient method to map AWS SQS [`sqs::Error`] to [`CoffeeShopError`].
    pub fn from_aws_sqs_error(error: sqs::Error, config: &dyn HasSQSConfiguration) -> Self {
        match error {
            sqs::Error::InvalidAddress(sqs::InvalidAddress {
                message: msg_opt, ..
            }) => Self::InvalidConfiguration {
                field: "sqs_queue",
                message: msg_opt.unwrap_or_else(|| sqs::DEFAULT_ERROR_MESSAGE.to_string()),
            },
            sqs::Error::InvalidMessageContents(sqs::InvalidMessageContents {
                message: msg_opt,
                ..
            }) => Self::AWSSQSInvalidMessage(
                msg_opt.unwrap_or_else(|| sqs::DEFAULT_ERROR_MESSAGE.to_string()),
            ),
            sqs::Error::KmsAccessDenied(sqs::KmsAccessDenied {
                message: msg_opt, ..
            }) => Self::AWSCredentialsError(
                msg_opt.unwrap_or_else(|| sqs::DEFAULT_ERROR_MESSAGE.to_string()),
            ),
            sqs::Error::PurgeQueueInProgress(_) => {
                crate::error!(
                    "The AWS SQS queue {} had been purged in the last 60 seconds, and cannot be purged again right now. If you are running a test, please hold off for a minute.",
                    config.sqs_queue_url()
                );
                Self::AWSSQSQueueBeingPurged
            }
            sqs::Error::QueueDoesNotExist(sqs::QueueDoesNotExist { .. }) => {
                Self::AWSQueueDoesNotExist(config.sqs_queue_url().to_owned())
            }
            err => Self::AWSSQSResponseError(Box::new(err)),
        }
    }

    /// Convenient method to map AWS [`dynamodb::Error`] to [`CoffeeShopError`].
    pub fn from_aws_dynamodb_error(
        error: dynamodb::Error,
        config: &dyn HasDynamoDBConfiguration,
    ) -> Self {
        match error {
            dynamodb::Error::ConditionalCheckFailedException(
                dynamodb::ConditionalCheckFailedException { message, item, .. },
            ) => Self::AWSDynamoDBMalformedItem(format!(
                "Conditional check failed for {item:?}: {message:#?}"
            )),
            dynamodb::Error::DuplicateItemException(dynamodb::DuplicateItemException {
                message,
                ..
            }) => Self::AWSDynamoDBDuplicateItem(
                message.unwrap_or("(no details provided)".to_owned()),
            ),
            dynamodb::Error::GlobalTableNotFoundException(_) => {
                Self::AWSDynamoDBTableDoesNotExist(config.dynamodb_table().to_owned())
            }
            dynamodb::Error::IndexNotFoundException(_) => Self::InvalidConfiguration {
                field: "dynamodb_partition_key",
                message: format!(
                    "`{}` is not a valid index on the DynamoDB Table of {}.",
                    config.dynamodb_partition_key(),
                    config.dynamodb_table()
                ),
            },
            dynamodb::Error::InvalidEndpointException(_) => Self::InvalidConfiguration {
                field: "dynamodb_table",
                message: format!(
                    "Invalid endpoint for DynamoDB; please verify the table: {}",
                    config.dynamodb_table()
                ),
            },
            dynamodb::Error::ItemCollectionSizeLimitExceededException(_) => {
                crate::error!(
                    "The item collection size limit has been exceeded for the table: {}",
                    config.dynamodb_table()
                );
                Self::AWSDynamoDBCollectionOversize
            }
            dynamodb::Error::LimitExceededException(dynamodb::LimitExceededException {
                message,
                ..
            }) => {
                crate::error!(
                    "An AWS DynamoDB rate limit has been exceeded for the table {}: {}",
                    config.dynamodb_table(),
                    message.unwrap_or("(no details provided)".to_owned())
                );
                Self::AWSDynamoDBRateLimitExceeded
            }
            dynamodb::Error::TableNotFoundException(_) => {
                Self::AWSDynamoDBTableDoesNotExist(config.dynamodb_table().to_owned())
            }
            dynamodb::Error::ProvisionedThroughputExceededException(_) => {
                Self::AWSDynamoDBRateLimitExceeded
            }
            dynamodb::Error::RequestLimitExceeded(_) => {
                crate::error!(
                    "Throughput limit for te AWS account has been exceeded for the table: {}",
                    config.dynamodb_table()
                );
                Self::AWSDynamoDBRateLimitExceeded
            }
            dynamodb::Error::ResourceNotFoundException(_) => {
                Self::AWSDynamoDBTableDoesNotExist(config.dynamodb_table().to_owned())
            }
            dynamodb::Error::TransactionCanceledException(
                dynamodb::TransactionCanceledException {
                    message,
                    cancellation_reasons,
                    ..
                },
            ) => Self::AWSDynamoDBItemOperationError {
                kind: "TransactionCanceledException".to_string(),
                message: format!(
                    "Transaction canceled for the table {}:\nCancellation reasons: {cancellation_reasons:?}\nMessage: {message:#?}",
                    config.dynamodb_table(),
                ),
            },
            // Fallback to the generic error.
            err => Self::AWSDynamoDBResponseError(Box::new(err)),
        }
    }

    /// This method returns the appropriate HTTP status code for the error.
    ///
    /// Some of these errors will not be encountered as a result of a request,
    /// but are included for completeness.
    ///
    /// If not found, it will return a [`http::StatusCode::INTERNAL_SERVER_ERROR`].
    pub fn status_code(&self) -> http::StatusCode {
        match self {
            Self::AWSConfigIncomplete(_) => http::StatusCode::UNAUTHORIZED,
            Self::AWSQueueDoesNotExist(_) => http::StatusCode::BAD_GATEWAY,
            Self::InvalidConfiguration { .. } => http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidHeader { .. } => http::StatusCode::NOT_ACCEPTABLE,
            Self::InvalidMethod => http::StatusCode::METHOD_NOT_ALLOWED,
            Self::InvalidMulticastAddress(_) => http::StatusCode::BAD_REQUEST,
            Self::InvalidMulticastMessage { .. } => http::StatusCode::BAD_REQUEST,
            Self::InvalidRoute(_) => http::StatusCode::NOT_FOUND,
            Self::InvalidQueryOptions(_) => http::StatusCode::BAD_REQUEST,
            Self::InvalidPayload { .. } => http::StatusCode::UNPROCESSABLE_ENTITY,
            Self::MalformedJsonPayload(_) => http::StatusCode::BAD_REQUEST,
            Self::RetrieveTimeout(_) => http::StatusCode::REQUEST_TIMEOUT,
            Self::Base64EncodingOversize(_) => http::StatusCode::PAYLOAD_TOO_LARGE,
            Self::ProcessingError(ErrorSchema { status_code, .. }) => *status_code,
            Self::ErrorSchema(ErrorSchema { status_code, .. }) => *status_code,
            Self::AWSDynamoDBMalformedItem(_) => http::StatusCode::BAD_GATEWAY,
            _ => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// This method returns the kind of error as a string.
    pub fn kind(&self) -> &'static str {
        self.into()
    }

    pub fn as_error_schema(&self) -> ErrorSchema {
        match self {
            // Potentially unsafe! However it's best for the downstream maintainer to know
            // about this rather than silently failing.
            Self::ProcessingError(err) => err.clone(),
            Self::ErrorSchema(err) => err.clone(),
            Self::InvalidHeader { key, message } => ErrorSchema::new(
                self.status_code(),
                self.kind().to_string(),
                Some(serde_json::json!({
                    "message": "Unacceptable headers provided; please check the following headers.",
                    "headers": {
                        key.as_str(): message,
                    },
                })),
            ),
            _ => ErrorSchema::new(
                self.status_code(),
                self.kind().to_string(),
                Some(serde_json::json!({
                    "message": self.to_string(),
                })),
            ),
        }
    }

    /// Converts the error into a JSON object.
    pub fn as_json(&self) -> serde_json::Value {
        serde_json::to_value(self.as_error_schema()).unwrap_or_else(|_| panic!("Failed to serialize the `ErrorSchema` into JSON for the response. This should not be possible; please check your error type definition: {:?}",
                self))
    }

    /// Converts a [`BoxError`] into a [`CoffeeShopError`], which will always return
    /// a JSON response.
    pub fn from_axum_box_error(error: BoxError) -> Self {
        Self::ErrorSchema(ErrorSchema::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "BoxError".to_string(),
            Some(serde_json::json!({
                "message": error.to_string(),
            })),
        ))
    }
}

impl PartialEq for CoffeeShopError {
    fn eq(&self, other: &Self) -> bool {
        // When an error is serialized and deserialized, the kind will become
        // "ErrorSchema" for the downstream user. This is because the error is now
        // a message instead of an actual raised error; the error type is lost.
        // This comparison accounts for those errors.
        (self.kind() == other.kind()
            || self.kind() == "ErrorSchema"
            || other.kind() == "ErrorSchema")
            && self.as_json() == other.as_json()
    }
}

impl IntoResponse for CoffeeShopError {
    fn into_response(self) -> axum::response::Response<Body> {
        (
            self.status_code(),
            [
                (http::header::CONTENT_TYPE, "application/json"),
                (http::header::CACHE_CONTROL, "no-store"),
            ],
            Json(self.as_json()),
        )
            .into_response()
    }
}
