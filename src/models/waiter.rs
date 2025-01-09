//! A waiter is an async HTTP host that listens for incoming requests and insert them into
//! the specified AWS SQS queue.
//! For synchronous requests, the waiter will also asynchronously await a [`Notify`](tokio::sync::Notify)
//! event from the multicast channel and report back to the client when the request had been processed.

use std::{
    ops::Deref,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Weak,
    },
};

use axum::extract::{
    rejection::{JsonRejection, QueryRejection},
    Json, Query,
};
use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
};
use tokio::sync::Notify;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

use super::{
    message::{self, QueryType},
    Machine, Order, Shop,
};
use crate::{errors::handling::IntoCoffeeShopError, helpers, CoffeeShopError};

const LOG_TARGET: &str = "coffeeshop::models::waiter";

/// A [`Waiter`] instance that acts as an async REST API host.
#[derive(Debug)]
pub struct Waiter<Q, I, O, F>
where
    Q: message::QueryType,
    I: serde::Serialize + serde::de::DeserializeOwned + Send + Sync,
    O: serde::Serialize + serde::de::DeserializeOwned + Send + Sync,
    F: Machine<Q, I, O>,
{
    /// The back reference to the shop that this waiter is serving.
    pub shop: Weak<Shop<Q, I, O, F>>,

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
    Q: message::QueryType + 'static,
    I: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + 'static,
    O: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + 'static,
    F: Machine<Q, I, O>,
{
    /// Create a new [`Waiter`] instance.
    pub fn new(shop: Weak<Shop<Q, I, O, F>>) -> Self {
        Self {
            shop,
            request_count: Arc::new(AtomicUsize::new(0)),
            start_time: tokio::time::Instant::now(),
        }
    }

    /// Get a reference to the shop that this waiter is serving.
    pub fn shop(&self) -> Arc<Shop<Q, I, O, F>> {
        self.shop.upgrade().expect("Shop has been dropped; this should not be possible in normal use. Please report this to the maintainer.")
    }

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
                ticket_count: self.shop().orders.len(),
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

        self.create_and_retrieve_order(message::CombinedInput::new(params, Some(payload)), timeout)
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
    pub async fn create_order(
        &self,
        input: message::CombinedInput<Q, I>,
    ) -> Result<(message::Ticket, Arc<Order>), CoffeeShopError> {
        let shop = self.shop();

        // Validate the query prior to creating the ticket, to avoid unnecessary
        // processing of invalid requests.
        shop.coffee_machine.validate(&input.query, input.input.as_ref()).await
        .inspect_err(
            |err| crate::warn!(target: LOG_TARGET, "Validation failed, not pushing to SQS: {:#?}", err)
        )
        .map_err(CoffeeShopError::ErrorSchema)?;

        self.request_count.fetch_add(1, Ordering::Relaxed);

        let ticket = helpers::sqs::put_ticket(shop.deref(), input).await?;

        Ok((ticket.clone(), shop.spawn_order(ticket).await))
    }

    /// An internal method to retrieve the result of a ticket from the
    /// AWS SQS queue.
    pub async fn retrieve_order<'o>(&self, ticket: String) -> axum::response::Response {
        let start_time = self.start_time;

        let shop = self.shop();

        let order = shop
            .orders
            .get(&ticket)
            .map(|ref_item| ref_item.value().clone())
            .ok_or_else(|| CoffeeShopError::TicketNotFound(ticket.clone()));

        crate::info!(
            target: LOG_TARGET,
            "Took {} seconds to acquire read lock.",
            start_time.elapsed().as_secs_f32()
        );

        if let Err(err) = order {
            return err.into_response();
        }

        let order = order.unwrap();

        crate::info!(
            target: LOG_TARGET,
            "Waiting for order {} to complete...",
            ticket
        );

        // Wait for the order to complete.
        order
            .wait_and_fetch_when_complete::<O, _>(shop.deref())
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
    pub async fn retrieve_order_with_timeout(
        &self,
        ticket: String,
        timeout: Option<tokio::time::Duration>,
    ) -> axum::response::Response {
        let ticket_for_log = ticket.clone();

        if let Some(timeout) = timeout {
            tokio::select! {
                _ = tokio::time::sleep(timeout) => {
                    crate::error!(
                        target: LOG_TARGET,
                        "Timeout of {timeout} seconds reached while waiting for order {ticket} to complete.",
                        timeout = timeout.as_secs(),
                        ticket = ticket_for_log,
                    );
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
    ///
    /// The `timeout` parameter is used to set a timeout for the processing and
    /// retrieval of the ticket only; the creation of the ticket is not affected.
    pub async fn create_and_retrieve_order(
        &self,
        input: message::CombinedInput<Q, I>,
        timeout: Option<tokio::time::Duration>,
    ) -> axum::response::Response {
        match self.create_order(input).await {
            Ok((ticket, _order)) => self.retrieve_order_with_timeout(ticket, timeout).await,
            Err(err) => err.into_response(),
        }
    }
}

/// Implementation for the waiter where the query types are 'static.
impl<Q, I, O, F> Waiter<Q, I, O, F>
where
    Q: message::QueryType + 'static,
    I: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + 'static,
    O: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + 'static,
    F: Machine<Q, I, O> + 'static,
{
    /// Start an [`axum`] app and serve incoming requests.
    pub async fn serve(
        self: &Arc<Self>,
        additional_routes: impl Iterator<
            Item = (
                &'static str,
                axum::routing::method_routing::MethodRouter<()>,
            ),
        >,
        shutdown_signal: Arc<Notify>,
        max_execution_time: Option<tokio::time::Duration>,
    ) -> Result<(), CoffeeShopError> {
        let mut app = axum::Router::new()
            .route(
                "/status",
                axum::routing::get({
                    let arc_self = Arc::clone(self);

                    || async move { arc_self.status().await }
                }),
            )
            .route(
                "/request",
                axum::routing::post({
                    let arc_self = Arc::clone(self);

                    // Add Error handling to the request handler.
                    |query_result: Result<Query<Q>, QueryRejection>,
                     json_result: Result<Json<I>, JsonRejection>| async move {
                        match (query_result, json_result) {
                            (Err(query_rejection), _) => {
                                let err = query_rejection.into_coffeeshop_error();

                                crate::warn!(
                                    target: LOG_TARGET,
                                    "Query rejection for /request: {:#?}",
                                    err
                                );

                                err.into_response()
                            }
                            (_, Err(json_rejection)) => {
                                let err = json_rejection.into_coffeeshop_error();

                                crate::warn!(
                                    target: LOG_TARGET,
                                    "JSON rejection for /request: {:#?}",
                                    err
                                );

                                err.into_response()
                            }
                            (Ok(Query(params)), Ok(json)) => {
                                // Check if the request is synchronous or asynchronous,
                                // and call the appropriate method.
                                // Pre-convert all errors to responses, so that the typing
                                // is consistent.
                                if params.is_async() {
                                    crate::info!(
                                        target: LOG_TARGET,
                                        "Received an asynchronous request.",
                                    );
                                    arc_self
                                        .async_request(Query(params), json)
                                        .await
                                        .into_response()
                                } else {
                                    crate::info!(
                                        target: LOG_TARGET,
                                        "Received a blocking request.",
                                    );
                                    arc_self.request(Query(params), json).await.into_response()
                                }
                            }
                        }
                    }
                }),
            )
            .route(
                "/retrieve",
                axum::routing::get({
                    let arc_self = Arc::clone(self);

                    |query_result: Result<Query<message::TicketQuery>, QueryRejection>| async move {
                        match query_result {
                            Err(rejection) => {
                                let err = rejection.into_coffeeshop_error();

                                crate::warn!(
                                    target: LOG_TARGET,
                                    "Query rejection for /retrieve: {:#?}",
                                    err
                                );

                                err.into_response()
                            }
                            Ok(query) => arc_self.async_retrieve(query).await.into_response(),
                        }
                    }
                }),
            );

        // Add additional routes to the app.
        app = additional_routes.fold(app, |app, (path, handler)| app.route(path, handler));

        // 404 Fallback.
        app = app.fallback(|uri| async {
            crate::warn!(
                target: LOG_TARGET,
                "Received a request for an invalid route: {}",
                uri
            );

            CoffeeShopError::InvalidRoute(uri)
        });

        // Method not allowed fallback.
        app = app.method_not_allowed_fallback(|| async {
            crate::warn!(
                target: LOG_TARGET,
                "Received a request with an invalid method.",
            );

            CoffeeShopError::InvalidMethod
        });

        // Add the trace and timeout layers to the app.
        if let Some(max_execution_time) = max_execution_time {
            app = app.layer((
                TraceLayer::new_for_http(),
                TimeoutLayer::new(max_execution_time),
            ))
        }

        let socket_addr = self.shop().config.host_addr();
        let listener = tokio::net::TcpListener::bind(&socket_addr)
            .await
            .map_err(|err| {
                CoffeeShopError::ListenerCreationFailure(
                    err.to_string(),
                    self.shop().config.host_addr(),
                )
            })?;

        let server = axum::serve(listener, app)
            .with_graceful_shutdown(async move { shutdown_signal.notified().await });

        let result = tokio::try_join!(server, async {
            crate::info!(
                target: LOG_TARGET,
                "Waiter is listening on {socket_addr:?}. Press Ctrl+C to stop.",
            );

            Ok(())
        })
        // Remove the server start logger from the error.
        .map(|(result, _)| result)
        .map_err(CoffeeShopError::from_server_io_error);

        crate::warn!(
            target: LOG_TARGET,
            "The waiter has stopped serving requests."
        );

        result
    }
}
