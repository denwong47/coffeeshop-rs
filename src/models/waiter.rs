//! A waiter is an async HTTP host that listens for incoming requests and insert them into
//! the specified AWS SQS queue.
//! For synchronous requests, the waiter will also asynchronously await a [`Notify`](tokio::sync::Notify)
//! event from the multicast channel and report back to the client when the request had been processed.

use axum::response::IntoResponse;
use axum::extract::{Json, Query};

/// A [`Waiter`] instance that acts as an async HTTP host.
#[derive(Debug)]
pub struct Waiter {}

impl Waiter {
    /// Handler for incoming requests.
    async fn request<Q, I>(
        Query(params): Query<Q>,
        Json(payload): Json<I>,
    ) -> impl IntoResponse {
        todo!()
    }

    /// Handler for asynchronous requests.
    async fn async_request<Q, I>(
        Query(params): Query<Q>,
        Json(payload): Json<I>,
    ) -> impl IntoResponse {
        todo!()
    }
}
