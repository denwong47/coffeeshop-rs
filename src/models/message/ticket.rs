use super::{QueryType, ResponseMetadata};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

/// A ticket is a unique identifier for a request that is processed asynchronously.
///
/// This contains the AWS SQS message ID, and the type must be a string.
pub type Ticket = String;

/// A query structure to retrieve the result of a ticket.
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TicketQuery {
    pub ticket: Ticket,
    #[serde_as(as = "Option<serde_with::DurationSecondsWithFrac<f64>>")]
    pub timeout: Option<tokio::time::Duration>,
}

/// Implement the [`QueryType`] trait for [`TicketQuery`].
impl QueryType for TicketQuery {
    fn get_timeout(&self) -> Option<tokio::time::Duration> {
        self.timeout
    }
}

/// A response structure to return the result of a ticket.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TicketResponse {
    pub metadata: ResponseMetadata,
    pub ticket: Ticket,
}

impl TicketResponse {
    /// Create a new [`TicketResponse`] instance.
    pub fn new(metadata: ResponseMetadata, ticket: Ticket) -> Self {
        Self { metadata, ticket }
    }

    /// Create a new [`TicketResponse`] instance with the default metadata.
    pub fn new_from_ticket(start_time: &tokio::time::Instant, ticket: Ticket) -> Self {
        Self {
            metadata: ResponseMetadata::new(start_time),
            ticket,
        }
    }
}

impl IntoResponse for TicketResponse {
    fn into_response(self) -> axum::response::Response<axum::body::Body> {
        (
            axum::http::StatusCode::ACCEPTED,
            [
                (axum::http::header::CONTENT_TYPE, "application/json"),
                (axum::http::header::CACHE_CONTROL, "no-store"),
            ],
            axum::Json(self),
        )
            .into_response()
    }
}
