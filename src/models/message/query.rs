#[cfg(doc)]
use axum::http;

#[cfg(doc)]
use tokio::time::Duration;

/// [`QueryType`] is a trait that defines the methods that a query type must implement.
///
/// This allows the designer to customise the query parameters to their needs, while
/// maintaining a standardised interface for the waiter to know certain information about
/// the query.
pub trait QueryType: serde::de::DeserializeOwned {
    /// Get the timeout for the query.
    ///
    /// This is used to determine how long the waiter should wait for a response
    /// before issuing a [`http::StatusCode::REQUEST_TIMEOUT`] response.
    ///
    /// While a [`None`] value is allowed, it is strongly recommended to enforce a
    /// [`Some<Duration>`] value to prevent the waiter from waiting indefinitely.
    fn get_timeout(&self) -> Option<tokio::time::Duration>;
}
