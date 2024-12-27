//! A waiter is an async HTTP host that listens for incoming requests and insert them into
//! the specified AWS SQS queue.
//! For synchronous requests, the waiter will also asynchronously await a [`Notify`](tokio::sync::Notify)
//! event from the multicast channel and report back to the client when the request had been processed.

#![allow(unused_variables)]

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::Notify;

use axum::extract::{Json, Query};
use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
};

use super::{
    message::{self, QueryType},
    Machine, Shop, Ticket,
};
use crate::CoffeeShopError;

/// A [`Waiter`] instance that acts as an async REST API host.
#[derive(Debug)]
pub struct Waiter<Q, I, O, F>
where
    Q: message::QueryType,
    I: serde::de::DeserializeOwned + serde::Serialize,
    O: serde::Serialize + serde::de::DeserializeOwned,
    F: Machine<I, O>,
{
    /// The back reference to the shop that this waiter is serving.
    pub shop: Arc<Shop<I, O, F>>,

    /// The total amount of historical requests processed.
    /// Only the [`request`](Self::request) and [`async_request`](Self::async_request) methods
    /// will increment this counter.
    ///
    /// Internally, this is done by [`create_ticket`](Self::create_ticket).
    pub request_count: Arc<AtomicUsize>,
    pub start_time: tokio::time::Instant,
    _phantom: std::marker::PhantomData<(Q, I, O)>,
}

impl<Q, I, O, F> Waiter<Q, I, O, F>
where
    Q: message::QueryType,
    I: serde::de::DeserializeOwned + serde::Serialize,
    O: serde::Serialize + serde::de::DeserializeOwned,
    F: Machine<I, O>,
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
                ticket_count: self.shop.tickets.read().await.len(),
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
            .map(|(ticket, output)| message::OutputResponse {
                ticket,
                metadata: message::ResponseMetadata::new(&self.start_time),
                output,
            })
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
        self.create_ticket(message::CombinedInput {
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

        self.retrieve_ticket_timeout(params.ticket, timeout)
            .await
            .map(|(ticket, output)| message::OutputResponse {
                ticket,
                metadata: message::ResponseMetadata::new(&self.start_time),
                output,
            })
    }

    /// An internal method to create a new ticket on the AWS SQS queue,
    /// then return the [`Notify`] instance to await the result.
    async fn create_ticket(
        &self,
        input: message::CombinedInput<Q, I>,
    ) -> Result<(message::Ticket, Arc<Notify>), CoffeeShopError> {
        self.request_count.fetch_add(1, Ordering::Relaxed);

        todo!()
    }

    /// An internal method to retrieve the result of a ticket from the
    /// AWS SQS queue.
    async fn retrieve_ticket(&self, ticket: String) -> Result<(Ticket, O), CoffeeShopError> {
        todo!()
    }

    /// An internal method to retrieve the result of a ticket with a timeout;
    /// if the timeout is reached, an [`CoffeeShopError::RetrieveTimeout`] is returned.
    async fn retrieve_ticket_timeout(
        &self,
        ticket: String,
        timeout: Option<tokio::time::Duration>,
    ) -> Result<(Ticket, O), CoffeeShopError> {
        if let Some(timeout) = timeout {
            tokio::select! {
                _ = tokio::time::sleep(timeout) => {
                    Err(CoffeeShopError::RetrieveTimeout(timeout))
                }
                result = self.retrieve_ticket(ticket) => {
                    result
                }
            }
        } else {
            self.retrieve_ticket(ticket).await
        }
    }

    /// An internal method to create a new ticket, wait for the result,
    /// then return the result to the client.
    async fn create_and_retrieve_ticket(
        &self,
        input: message::CombinedInput<Q, I>,
        timeout: Option<tokio::time::Duration>,
    ) -> Result<(Ticket, O), CoffeeShopError> {
        let (ticket, notify) = self.create_ticket(input).await?;

        if let Some(timeout) = timeout {
            tokio::select! {
                _ = notify.notified() => {
                    // We can take a bit of risk here given that we already know that
                    // the notification had been received.
                    self.retrieve_ticket(ticket).await
                }
                _ = tokio::time::sleep(timeout) => {
                    Err(CoffeeShopError::RetrieveTimeout(timeout))
                }
            }
        } else {
            // No timeout.
            // Theoretically this is supported, but its a good idea to enforce a timeout.
            notify.notified().await;
            self.retrieve_ticket(ticket).await
        }
    }
}
