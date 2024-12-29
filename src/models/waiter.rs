//! A waiter is an async HTTP host that listens for incoming requests and insert them into
//! the specified AWS SQS queue.
//! For synchronous requests, the waiter will also asynchronously await a [`Notify`](tokio::sync::Notify)
//! event from the multicast channel and report back to the client when the request had been processed.

use std::{
    ops::Deref,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use axum::extract::{Json, Query};
use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
};

use super::{
    message::{self, QueryType},
    Machine, Order, Shop,
};
use crate::{helpers, CoffeeShopError};

/// The maximum timeout for a waiter to wait for a ticket to be processed.
const MAX_WAITER_TIMEOUT: tokio::time::Duration = tokio::time::Duration::from_secs(24 * 60 * 60);

/// A [`Waiter`] instance that acts as an async REST API host.
#[derive(Debug)]
pub struct Waiter<Q, I, O, F>
where
    Q: message::QueryType,
    I: serde::Serialize + serde::de::DeserializeOwned,
    O: serde::Serialize + serde::de::DeserializeOwned + Send + Sync,
    F: Machine<Q, I, O>,
{
    /// The back reference to the shop that this waiter is serving.
    pub shop: Arc<Shop<Q, I, O, F>>,

    /// The total amount of historical requests processed.
    /// Only the [`request`](Self::request) and [`async_request`](Self::async_request) methods
    /// will increment this counter.
    ///
    /// Internally, this is done by [`create_ticket`](Self::create_ticket).
    pub request_count: Arc<AtomicUsize>,
    pub start_time: tokio::time::Instant,
}

impl<Q, I, O, F> Waiter<Q, I, O, F>
where
    Q: message::QueryType,
    I: serde::Serialize + serde::de::DeserializeOwned,
    O: serde::Serialize + serde::de::DeserializeOwned + Send + Sync,
    F: Machine<Q, I, O>,
{
    /// `GET` Handler for getting the status of the waiter.
    pub async fn status(&self) -> impl IntoResponse {
        (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, "application/json"),
                (header::CACHE_CONTROL, "no-store"),
            ],
            Json(message::StatusResponse {
                metadata: message::ResponseMetadata::new(&self.start_time),
                request_count: self.request_count.load(Ordering::Relaxed),
                ticket_count: self.shop.orders.read().await.len(),
            }),
        )
    }

    /// `POST` Handler for incoming requests.
    pub async fn request(
        &self,
        Query(params): Query<Q>,
        Json(payload): Json<I>,
    ) -> impl IntoResponse {
        let timeout = params.get_timeout();

        self.create_and_retrieve_ticket(message::CombinedInput::new(params, Some(payload)), timeout)
            .await
    }

    /// `POST` Handler for asynchronous requests.
    ///
    /// This immediately returns a `202 Accepted` response with
    /// the ticket ID as the body.
    pub async fn async_request(
        &self,
        Query(params): Query<Q>,
        Json(payload): Json<I>,
    ) -> impl IntoResponse {
        self.create_order(message::CombinedInput {
            query: params,
            input: Some(payload),
        })
        .await
        .map(|(ticket, _)| message::TicketResponse {
            ticket,
            metadata: message::ResponseMetadata::new(&self.start_time),
        })
    }

    /// A `GET` request to fetch results from a previously processed request.
    pub async fn async_retrieve(
        &self,
        Query(params): Query<message::TicketQuery>,
    ) -> impl IntoResponse {
        let timeout = params.get_timeout();

        self.retrieve_order_with_timeout(params.ticket, timeout)
            .await
    }

    /// An internal method to create a new ticket on the AWS SQS queue,
    /// then return the [`Order`] instance to await the result.
    async fn create_order(
        &self,
        input: message::CombinedInput<Q, I>,
    ) -> Result<(message::Ticket, Arc<Order>), CoffeeShopError> {
        self.request_count.fetch_add(1, Ordering::Relaxed);

        let ticket =
            helpers::sqs::put_ticket(self.shop.deref(), input, &self.shop.temp_dir).await?;

        Ok((ticket.clone(), self.shop.spawn_order(ticket).await))
    }

    /// An internal method to retrieve the result of a ticket from the
    /// AWS SQS queue.
    async fn retrieve_order<'o>(&self, ticket: String) -> axum::response::Response {
        let start_time = self.start_time;

        let order = self
            .shop
            .orders
            .read()
            .await
            .get(&ticket)
            .ok_or_else(|| CoffeeShopError::TicketNotFound(ticket.clone()))
            .map(Arc::clone);

        if let Err(err) = order {
            return err.into_response();
        }

        let order = order.unwrap();

        order
            .wait_until_complete::<O, _>(self.shop.deref())
            .await
            .map(|result| {
                result.map(|output| {
                    // Use the `OutputResponse` to create a response.
                    message::OutputResponse::new(ticket, &output, &start_time).into_response()
                })
            })
            // Convert any remaining errors into responses.
            .into_response()
    }

    /// An internal method to retrieve the result of a ticket with a timeout;
    /// if the timeout is reached, an [`CoffeeShopError::RetrieveTimeout`] is returned.
    async fn retrieve_order_with_timeout(
        &self,
        ticket: String,
        timeout: Option<tokio::time::Duration>,
    ) -> axum::response::Response {
        if let Some(timeout) = timeout {
            tokio::select! {
                _ = tokio::time::sleep(timeout) => {
                    Err::<(), _>(CoffeeShopError::RetrieveTimeout(timeout)).into_response()
                }
                result = self.retrieve_order(ticket) => {
                    result
                }
            }
        } else {
            self.retrieve_order(ticket).await
        }
    }

    /// An internal method to create a new ticket, wait for the result,
    /// then return the result to the client.
    async fn create_and_retrieve_ticket(
        &self,
        input: message::CombinedInput<Q, I>,
        timeout: Option<tokio::time::Duration>,
    ) -> axum::response::Response {
        match self.create_order(input).await {
            Ok((ticket, order)) => {
                let timeout = timeout.unwrap_or(MAX_WAITER_TIMEOUT);

                tokio::select! {
                    result = order.wait_until_complete::<O, _>(self.shop.deref()) => result.map(
                        |result| result.map(
                            // Use the `OutputResponse` to create a response.
                            |output| message::OutputResponse::new(
                                ticket,
                                &output,
                                &self.start_time,
                            ).into_response()
                        )
                    // Convert any remaining errors into responses.
                    ).into_response(),
                    _ = tokio::time::sleep(timeout) => {
                        Err::<(), _>(CoffeeShopError::RetrieveTimeout(timeout)).into_response()
                    }
                }
            }
            Err(err) => err.into_response(),
        }
    }
}
