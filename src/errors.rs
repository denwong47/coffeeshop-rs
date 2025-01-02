use axum::{body::Body, http, response::IntoResponse, Json};
use thiserror::Error;

use std::net::{IpAddr, SocketAddr};

#[cfg(doc)]
use crate::models::{Barista, Waiter};

use crate::{helpers::sqs::HasSQSConfiguration, models::Ticket};

/// Re-exports necessary for the error handling of SQS SDK.
mod sqs {
    pub const DEFAULT_ERROR_MESSAGE: &str = "(No details provided)";

    pub use aws_sdk_sqs::operation::{
        receive_message::ReceiveMessageError, send_message::SendMessageError,
    };
    pub use aws_sdk_sqs::types::error::*;
}

/// The error type for validation errors.
///
/// This is a simple key-value pair where the key is the field that failed validation,
/// and the value is the error message.
///
/// The original value is not included in the error, as it could violate lifetimes
/// as well as privacy.
///
/// By convention, if the error relates to the whole of
/// - query parameters, the key should be ``$query``, and
/// - the request body, the key should be ``$body``.
pub type ValidationError = std::collections::HashMap<String, String>;

/// The error type for exporting any error that occurs in this crate.
///
/// Since the [`Barista`]s have to serialize any errors to DynamoDB before a
/// [`Waiter`] can retrieve it, we need a standardised error type to ensure
/// that the errors can be logically
#[non_exhaustive]
#[derive(Error, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ErrorSchema {
    /// The HTTP status code to send to the client in the response.
    #[serde(with = "http_serde::status_code")]
    status_code: http::StatusCode,

    /// An identifier for the type of error that occurred in PascalCase, e.g.
    /// `InvalidConfiguration`.
    error: String,

    /// Additional details for the error.
    ///
    /// These are returned to the user directly as part of the error response.
    /// This crate will not attempt to interpret the contents of this field.
    ///
    /// It is encouraged for this field to contain the key "message" with a human-readable
    /// error message.
    details: Option<serde_json::Value>,
}

/// The error type for the Coffee Machine.
///
/// This is for downstream implementers to use as the error type for their Coffee Machine.
pub type CoffeeMachineError = ErrorSchema;

impl ErrorSchema {
    /// Create a new instance of [`ErrorSchema`].
    pub fn new(
        status_code: http::StatusCode,
        error: String,
        details: Option<serde_json::Value>,
    ) -> Self {
        Self {
            status_code,
            error,
            details,
        }
    }
}

impl IntoResponse for ErrorSchema {
    fn into_response(self) -> axum::response::Response<Body> {
        (
            self.status_code,
            [
                (http::header::CONTENT_TYPE, "application/json"),
                (http::header::CACHE_CONTROL, "no-store"),
            ],
            Json(serde_json::to_value(&self).expect(
                // Potentially unsafe! This should however be unreachable.
                "Failed to serialize the `ErrorSchema` into JSON for the response. This should not be possible; please check your error type definition.",
            )),
        )
            .into_response()
    }
}

impl std::fmt::Display for ErrorSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.status_code
                .canonical_reason()
                .unwrap_or(&format!("{:?}", self.status_code.as_u16())),
            self.error
        )
    }
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

    #[error("DynamoDB item is found malformed: {0}")]
    DynamoDBMalformedItem(String),

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
            CoffeeShopError::InvalidMulticastAddress(_) => http::StatusCode::BAD_REQUEST,
            CoffeeShopError::InvalidMulticastMessage { .. } => http::StatusCode::BAD_REQUEST,
            CoffeeShopError::RetrieveTimeout(_) => http::StatusCode::REQUEST_TIMEOUT,
            CoffeeShopError::Base64EncodingOversize(_) => http::StatusCode::PAYLOAD_TOO_LARGE,
            CoffeeShopError::ProcessingError(ErrorSchema { status_code, .. }) => *status_code,
            CoffeeShopError::ErrorSchema(ErrorSchema { status_code, .. }) => *status_code,
            CoffeeShopError::DynamoDBMalformedItem(_) => http::StatusCode::BAD_GATEWAY,
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
