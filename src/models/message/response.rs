use axum::{body::Body, http, response::IntoResponse, Json};
use serde::{
    de::{self, DeserializeOwned},
    Deserialize, Serialize,
};

use super::{ResponseMetadata, Ticket};

/// Response message for the output of a request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OutputResponse<'o, O>
where
    O: Serialize,
{
    pub ticket: Ticket,
    pub metadata: ResponseMetadata,
    pub output: &'o O,
}

impl<'o, O> OutputResponse<'o, O>
where
    O: Serialize,
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

/// The exported version of the [`OutputResponse`] structure, which owns the output.
///
/// This is only for clients calling the Coffee Shop API to deserialize the response;
/// internally, only the unit tests use this structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputResponseExport<O> {
    pub ticket: Ticket,
    pub metadata: ResponseMetadata,
    pub output: O,
}

impl<'de, 'o, O> Deserialize<'de> for OutputResponseExport<O>
where
    'o: 'de,
    O: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct OutputResponseHelper {
            ticket: Ticket,
            metadata: ResponseMetadata,
            // Use `serde_json::Value` as a staging area for `deserialize_any`
            // to handle the output field.
            output: serde_json::Value,
        }

        let OutputResponseHelper {
            ticket,
            metadata,
            output,
        } = OutputResponseHelper::deserialize(deserializer)?;
        Ok(Self {
            ticket,
            metadata,
            output: serde_json::from_value(output)
                .map_err(|e| de::Error::custom(format!("failed to deserialize output: {}", e)))?,
        })
    }
}
