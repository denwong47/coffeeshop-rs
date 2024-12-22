use axum::{body::Body, http, response::IntoResponse, Json};

use super::{ResponseMetadata, Ticket};

/// Response message for the output of a request.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct OutputResponse<O>
where
    O: serde::Serialize,
{
    pub ticket: Ticket,
    pub metadata: ResponseMetadata,
    pub output: O,
}

impl<O> IntoResponse for OutputResponse<O>
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
