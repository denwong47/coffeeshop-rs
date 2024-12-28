use axum::{body::Body, http, response::IntoResponse, Json};

use super::{ResponseMetadata, Ticket};

/// Response message for the output of a request.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct OutputResponse<'o, O>
where
    O: serde::Serialize,
{
    pub ticket: Ticket,
    pub metadata: ResponseMetadata,
    pub output: &'o O,
}

impl<'o, O> OutputResponse<'o, O>
where
    O: serde::Serialize,
{
    /// Create a new [`OutputResponse`] instance.
    pub fn new(ticket: Ticket, output: &'o O, start_time: &tokio::time::Instant) -> Self {
        Self {
            ticket,
            metadata: ResponseMetadata::new(start_time),
            output,
        }
    }
}

impl<O> IntoResponse for OutputResponse<'_, O>
where
    O: serde::Serialize,
{
    fn into_response(self) -> axum::response::Response<Body> {
        (
            http::StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, "application/json")],
            Json(self),
        )
            .into_response()
    }
}
