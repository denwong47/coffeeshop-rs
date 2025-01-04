use axum::{body::Body, http, response::IntoResponse, Json};
use thiserror::Error;

#[cfg(doc)]
use crate::models::{Barista, Waiter};

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
    pub status_code: http::StatusCode,

    /// An identifier for the type of error that occurred in PascalCase, e.g.
    /// `InvalidConfiguration`.
    pub error: String,

    /// Additional details for the error.
    ///
    /// These are returned to the user directly as part of the error response.
    /// This crate will not attempt to interpret the contents of this field.
    ///
    /// It is encouraged for this field to contain the key "message" with a human-readable
    /// error message.
    pub details: Option<serde_json::Value>,
}

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
