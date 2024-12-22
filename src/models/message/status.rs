use super::ResponseMetadata;

/// Status report of the waiter.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatusResponse {
    /// Metadata of the response.
    pub metadata: ResponseMetadata,

    /// dequest count.
    pub request_count: usize,

    /// Ticket count.
    pub ticket_count: usize,
}
