use axum::{body::Body, http, response::IntoResponse, Json};
use thiserror::Error;

use std::net::IpAddr;

/// Re-exports necessary for the error handling of SQS SDK.
mod sqs {
    pub const DEFAULT_ERROR_MESSAGE: &str = "(No details provided)";

    pub use aws_sdk_sqs::operation::send_message::SendMessageError;
    pub use aws_sdk_sqs::types::error::*;
}

#[derive(Error, Debug)]
pub enum CoffeeShopError {
    #[error("Invalid configuration for {field}: {message}")]
    InvalidConfiguration {
        field: &'static str,
        message: String,
    },

    #[error("{0:?} is not a valid multicast address.")]
    InvalidMulticastAddress(IpAddr),

    #[error("Received an invalid {field} in MulticastMessage: {value}")]
    InvalidMulticastMessage { field: &'static str, value: String },

    #[error("HTTP Host failed: {0}")]
    AxumError(axum::Error),

    #[error("Could not serialize the payload: {0}")]
    ResultBinaryConversionError(#[from] Box<bincode::ErrorKind>),

    #[error("Could not compress/decompress the payload: {0}")]
    ResultBinaryCompressionError(#[from] gzp::GzpError),

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

    #[error("Timed out awaiting results after {0:?} seconds")]
    RetrieveTimeout(std::time::Duration),

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

    #[error("Message from AWS SQS had already been completed, and cannot be {0} again.")]
    AWSSQSStagedReceiptAlreadyCompleted(&'static str),

    #[error("AWS responded with unexpected data: {0}")]
    UnexpectedAWSResponse(String),

    /// Generic AWS SDK error.
    ///
    /// Use this as a last resort, as it is not specific to any SDK.
    #[error("AWS SDK Error: {0}")]
    AWSSdkError(String),
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

    /// Convenient method to map AWS SQS SDK errors to [`CoffeeShopError`].
    pub fn from_aws_sqs_send_message_error(error: sqs::SendMessageError) -> Self {
        match error {
            sqs::SendMessageError::QueueDoesNotExist(sqs::QueueDoesNotExist {
                message: msg_opt,
                ..
            }) => CoffeeShopError::AWSQueueDoesNotExist(
                msg_opt.unwrap_or_else(|| sqs::DEFAULT_ERROR_MESSAGE.to_string()),
            ),
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
            _ => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// This method returns the kind of error as a string.
    pub fn kind(&self) -> String {
        format!("{:?}", self)
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
            Json(serde_json::json!({
                "error": self.kind(),
                "details": {
                    "message": self.to_string(),
                },
            })),
        )
            .into_response()
    }
}
