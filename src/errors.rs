use axum::{body::Body, http, response::IntoResponse, Json};
use thiserror::Error;

use std::net::IpAddr;

#[derive(Error, Debug)]
pub enum CoffeeShopError {
    #[error("Invalid configuration for {field}: {value}")]
    InvalidConfiguration { field: &'static str, value: String },

    #[error("{0:?} is not a valid multicast address.")]
    InvalidMulticastAddress(IpAddr),

    #[error("Received an invalid {field} in MulticastMessage: {value}")]
    InvalidMulticastMessage { field: &'static str, value: String },

    #[error("HTTP Host failed: {0}")]
    AxumError(axum::Error),

    #[error("Could not serialize the payload: {0}")]
    ResultBinaryConversionError(#[from] Box<bincode::ErrorKind>),

    #[error("Could not compress the payload: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Timed out awaiting results after {0:?} seconds")]
    RetrieveTimeout(std::time::Duration),
}

impl CoffeeShopError {
    /// This method returns the appropriate HTTP status code for the error.
    ///
    /// Some of these errors will not be encountered as a result of a request,
    /// but are included for completeness.
    ///
    /// If not found, it will return a [`http::StatusCode::INTERNAL_SERVER_ERROR`].
    pub fn status_code(&self) -> http::StatusCode {
        match self {
            CoffeeShopError::InvalidMulticastAddress(_) => http::StatusCode::BAD_REQUEST,
            CoffeeShopError::InvalidMulticastMessage { .. } => http::StatusCode::BAD_REQUEST,
            CoffeeShopError::RetrieveTimeout(_) => http::StatusCode::REQUEST_TIMEOUT,
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
