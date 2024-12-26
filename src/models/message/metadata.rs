use chrono::{DateTime, Utc};
use gethostname::gethostname as get_hostname;
use std::ffi::OsString;
use tokio::time::Duration;

/// Response Metadata, containing information about the host returning the response.
///
/// Mostly for debugging purposes.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ResponseMetadata {
    /// The IP address of the server.
    pub hostname: OsString,
    /// The timestamp of the response.
    pub timestamp: DateTime<Utc>,
    /// Server uptime in seconds.
    pub uptime: Duration,
}

impl ResponseMetadata {
    /// Create a new [`ResponseMetadata`] instance.
    pub fn new(start_time: &tokio::time::Instant) -> Self {
        Self {
            hostname: get_hostname(),
            timestamp: Utc::now(),
            uptime: start_time.elapsed(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "debug")]
    const LOG_TARGET: &str = "coffeeshop::models::message::metadata::tests";

    #[test]
    fn test_get_hostname() {
        let hostname = get_hostname();
        crate::debug!(target: LOG_TARGET, "Found hostname: {:?}", hostname);
        assert_ne!(hostname.len(), 0);
    }
}
