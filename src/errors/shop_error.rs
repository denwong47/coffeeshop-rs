use axum::{body::Body, http, response::IntoResponse, BoxError, Json};
use thiserror::Error;

use std::net::{IpAddr, SocketAddr};

use crate::{
    helpers::{dynamodb::HasDynamoDBConfiguration, sqs::HasSQSConfiguration},
    models::Ticket,
};

#[cfg(doc)]
use crate::models::{Barista, Waiter};

use super::{CoffeeMachineError, ErrorSchema};

/// For error handling of SQS SDK.
mod sqs {
    pub const DEFAULT_ERROR_MESSAGE: &str = "(No details provided)";

    pub use aws_sdk_sqs::operation::{
        receive_message::ReceiveMessageError, send_message::SendMessageError,
    };
    pub use aws_sdk_sqs::types::error::*;
}
/// For the error handling of DynamoDB SDK.
mod dynamodb {
    pub use aws_sdk_dynamodb::operation::put_item::PutItemError;
    pub use aws_sdk_dynamodb::types::error::*;
}

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
    ResultBinaryConversionError(#[from] Box<bincode::ErrorKind>),

    #[error("Could not compress/decompress the payload: {0}")]
    ResultBinaryCompressionError(#[from] gzp::GzpError),

    #[error("Temporary directory could not be created: {0}")]
    TempDirCreationFailure(String),

    #[error("Temporary file access failure at {path}: {reason}")]
    TempFileAccessFailure {
        path: std::path::PathBuf,
        reason: String,
    },

    #[error("The path for a temporary file is non-uniquely generated; this is improbable unless cleanup is not working. Please verify.")]
    NonUniqueTemporaryFile,

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

    #[error("The specified AWS DynamoDB Table does not exists. Please verify the table: {0}")]
    AWSDynamoDBTableDoesNotExist(String),

    #[error("DynamoDB item is found malformed: {0}")]
    AWSDynamoDBMalformedItem(String),

    #[error("AWS DynamoDB Provisioned Throughput Exceeded.")]
    AWSDynamoDBRateLimitExceeded,

    #[error("The specified AWS SQS queue URL does not exists. Please verify the URL: {0}")]
    AWSQueueDoesNotExist(String),

    #[error("AWS SQS Rejected the message: {0}; please verify the payload and try again.")]
    AWSSQSInvalidMessage(String),

    #[error("AWS SQS Queue is empty after waiting for {0:?}.")]
    AWSSQSQueueEmpty(tokio::time::Duration),

    #[error("Unexpected AWS SQS Send Message Error: {0:?}")]
    AWSSQSSendMessageError(#[from] Box<sqs::SendMessageError>),

    #[error("Unexpected AWS SQS Receive Message Error: {0:?}")]
    AWSSQSReceiveMessageError(#[from] Box<sqs::ReceiveMessageError>),

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
            CoffeeShopError::NonUniqueTemporaryFile
        } else {
            CoffeeShopError::IOError(error.kind(), error)
        }
    }

    /// Convenient method to create a [`CoffeeShopError::MulticastIOError`] variant from [`std::io::Error`].
    pub fn from_multicast_io_error(error: std::io::Error) -> Self {
        // We don't need to care about unique temporary files here.
        CoffeeShopError::MulticastIOError(error.kind(), error)
    }

    /// Convenient method to create a [`CoffeeShopError::HTTPServerError`] variant from [`std::io::Error`].
    pub fn from_server_io_error(error: std::io::Error) -> Self {
        // We don't need to care about unique temporary files here.
        CoffeeShopError::HTTPServerError(error.kind(), error)
    }

    /// Convenient method to map AWS SQS SDK errors from receiving messages to
    /// [`CoffeeShopError`].
    pub fn from_aws_sqs_receive_message_error(
        error: sqs::ReceiveMessageError,
        config: &dyn HasSQSConfiguration,
    ) -> Self {
        match error {
            sqs::ReceiveMessageError::QueueDoesNotExist(sqs::QueueDoesNotExist { .. }) => {
                CoffeeShopError::AWSQueueDoesNotExist(config.sqs_queue_url().to_owned())
            }
            sqs::ReceiveMessageError::InvalidAddress(sqs::InvalidAddress {
                message: msg_opt,
                ..
            }) => CoffeeShopError::InvalidConfiguration {
                field: "sqs_queue",
                message: msg_opt.unwrap_or_else(|| sqs::DEFAULT_ERROR_MESSAGE.to_string()),
            },
            sqs::ReceiveMessageError::KmsAccessDenied(sqs::KmsAccessDenied {
                message: msg_opt,
                ..
            }) => CoffeeShopError::AWSCredentialsError(
                msg_opt.unwrap_or_else(|| sqs::DEFAULT_ERROR_MESSAGE.to_string()),
            ),
            err => CoffeeShopError::AWSSQSReceiveMessageError(Box::new(err)),
        }
    }

    /// Convenient method to map AWS SQS SDK errors from sending messages to [`CoffeeShopError`].
    pub fn from_aws_sqs_send_message_error(
        error: sqs::SendMessageError,
        config: &dyn HasSQSConfiguration,
    ) -> Self {
        match error {
            sqs::SendMessageError::QueueDoesNotExist(sqs::QueueDoesNotExist { .. }) => {
                CoffeeShopError::AWSQueueDoesNotExist(config.sqs_queue_url().to_owned())
            }
            sqs::SendMessageError::InvalidMessageContents(sqs::InvalidMessageContents {
                message: msg_opt,
                ..
            }) => CoffeeShopError::AWSSQSInvalidMessage(
                msg_opt.unwrap_or_else(|| sqs::DEFAULT_ERROR_MESSAGE.to_string()),
            ),
            sqs::SendMessageError::InvalidAddress(sqs::InvalidAddress {
                message: msg_opt, ..
            }) => CoffeeShopError::InvalidConfiguration {
                field: "sqs_queue",
                message: msg_opt.unwrap_or_else(|| sqs::DEFAULT_ERROR_MESSAGE.to_string()),
            },
            sqs::SendMessageError::KmsAccessDenied(sqs::KmsAccessDenied {
                message: msg_opt,
                ..
            }) => CoffeeShopError::AWSCredentialsError(
                msg_opt.unwrap_or_else(|| sqs::DEFAULT_ERROR_MESSAGE.to_string()),
            ),
            err => CoffeeShopError::AWSSQSSendMessageError(Box::new(err)),
        }
    }

    /// Convenient method to map AWS DynamoDB SDK errors to [`CoffeeShopError`].
    pub fn from_aws_dynamodb_put_item_error(
        error: dynamodb::PutItemError,
        config: &dyn HasDynamoDBConfiguration,
    ) -> Self {
        match error {
            dynamodb::PutItemError::ResourceNotFoundException(_) => {
                CoffeeShopError::AWSDynamoDBTableDoesNotExist(config.dynamodb_table().to_owned())
            }
            dynamodb::PutItemError::InvalidEndpointException(_) => {
                CoffeeShopError::InvalidConfiguration {
                    field: "dynamodb_table",
                    message: format!(
                        "Invalid endpoint for DynamoDB; please verify the table: {}",
                        config.dynamodb_table()
                    ),
                }
            }
            dynamodb::PutItemError::ProvisionedThroughputExceededException(_) => {
                CoffeeShopError::AWSDynamoDBRateLimitExceeded
            }
            dynamodb::PutItemError::ConditionalCheckFailedException(
                dynamodb::ConditionalCheckFailedException { message, item, .. },
            ) => CoffeeShopError::AWSDynamoDBMalformedItem(format!(
                "Conditional check failed for {item:?}: {message:#?}"
            )),
            err => CoffeeShopError::AWSSdkError(format!("{:?}", err)),
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
            CoffeeShopError::AWSConfigIncomplete(_) => http::StatusCode::UNAUTHORIZED,
            CoffeeShopError::AWSQueueDoesNotExist(_) => http::StatusCode::BAD_GATEWAY,
            CoffeeShopError::InvalidConfiguration { .. } => http::StatusCode::INTERNAL_SERVER_ERROR,
            CoffeeShopError::InvalidHeader { .. } => http::StatusCode::NOT_ACCEPTABLE,
            CoffeeShopError::InvalidMulticastAddress(_) => http::StatusCode::BAD_REQUEST,
            CoffeeShopError::InvalidMulticastMessage { .. } => http::StatusCode::BAD_REQUEST,
            CoffeeShopError::InvalidRoute(_) => http::StatusCode::NOT_FOUND,
            CoffeeShopError::InvalidQueryOptions(_) => http::StatusCode::BAD_REQUEST,
            CoffeeShopError::InvalidPayload { .. } => http::StatusCode::UNPROCESSABLE_ENTITY,
            CoffeeShopError::MalformedJsonPayload(_) => http::StatusCode::BAD_REQUEST,
            CoffeeShopError::RetrieveTimeout(_) => http::StatusCode::REQUEST_TIMEOUT,
            CoffeeShopError::Base64EncodingOversize(_) => http::StatusCode::PAYLOAD_TOO_LARGE,
            CoffeeShopError::ProcessingError(ErrorSchema { status_code, .. }) => *status_code,
            CoffeeShopError::ErrorSchema(ErrorSchema { status_code, .. }) => *status_code,
            CoffeeShopError::AWSDynamoDBMalformedItem(_) => http::StatusCode::BAD_GATEWAY,
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
        CoffeeShopError::ErrorSchema(ErrorSchema::new(
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
