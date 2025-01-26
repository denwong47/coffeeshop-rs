//! Implementation of the [`CollectionPoint`] trait for the [`Shop`] struct.
//!
//! # Note
//!
//! This is a somewhat questionable design decision, as the [`Shop`] struct is already
//! quite large and complex, so arguably [`CollectionPoint`] could have been a
//! separate struct that contains a reference to a [`Shop`] instance.
//!
//! Due to lifetime issues, there are also some methods that could not be implemented
//! as part of the [`CollectionPoint`] trait, so they are implemented directly on the
//! [`Shop`] struct such as [`Shop::check_for_fulfilled_orders`] and
//! [`Shop::periodically_check_for_fulfilled_orders`].
use crate::{
    helpers::dynamodb::{self, HasDynamoDBConfiguration},
    models::{message, Machine, Orders, Shop, Ticket},
    CoffeeShopError,
};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use tokio::sync::Notify;

#[cfg(doc)]
use crate::models::{Barista, Order, Waiter};

const LOG_TARGET: &str = "coffeeshop::models::collection_point";

/// A [`CollectionPoint`] is a behaviour of a [`Shop`] that:
/// - Monitors the orders on DynamoDB that is flagged by the [`Waiter`]s
/// - Listens for Multicast messages from the [`Barista`]s from this and other [`Shop`]s
/// - Update the [`Order`]s in the [`Shop`] instance with the results from the [`Barista`]s
#[async_trait::async_trait]
pub trait CollectionPoint: HasDynamoDBConfiguration {
    /// Access the orders relevant to the collection point.
    fn orders(&self) -> &Orders;

    /// Purge stale orders from the collection point.
    ///
    /// Currently, this function never fails; the error type is reserved for future use.
    #[allow(unused_variables)]
    async fn purge_stale_orders(
        &self,
        // This needs to be implemented in the future at `Chain::advance` level.
        max_age: tokio::time::Duration,
    ) -> Result<(), CoffeeShopError> {
        self.orders().advance().await;
        Ok(())
    }

    /// Periodically purge stale orders from the collection point.
    ///
    /// This function will loop indefinitely until the program is terminated.
    async fn periodic_purge_stale_orders(
        &self,
        max_age: tokio::time::Duration,
        interval: tokio::time::Duration,
    ) {
        loop {
            tokio::time::sleep(interval).await;
            if let Err(err) = self.purge_stale_orders(max_age).await {
                crate::error!(
                    target: LOG_TARGET,
                    "Failed to purge stale orders from the collection point: {}",
                    err
                );
            }
        }
    }
}

#[async_trait::async_trait]
impl<Q, I, O, F> CollectionPoint for Shop<Q, I, O, F>
where
    Q: message::QueryType,
    I: Serialize + DeserializeOwned + Send + Sync,
    O: Serialize + DeserializeOwned + Send + Sync,
    F: Machine<Q, I, O>,
{
    /// Access the orders in the [`Shop`] instance.
    fn orders(&self) -> &Orders {
        &self.orders
    }
}

/// These methods could not form part of the [`CollectionPoint`] trait because they
/// uses the `self` reference in a way that requires too many lifetimes to be specified.
impl<Q, I, O, F> Shop<Q, I, O, F>
where
    Q: message::QueryType,
    I: Serialize + DeserializeOwned + Send + Sync,
    O: Serialize + DeserializeOwned + Send + Sync,
    F: Machine<Q, I, O>,
{
    /// Check DynamoDB for newly fulfilled [`Order`]s.
    ///
    /// # Note
    ///
    /// Internal function: this function is not meant to be called directly.
    pub async fn check_for_fulfilled_orders(&self) -> Result<(), CoffeeShopError> {
        let found_results: hashbrown::HashMap<Ticket, bool> = hashbrown::HashMap::from_iter(
            async {
                let orders = self.orders().iter().await;

                let unfulfilled_tickets = orders
                    .filter_map(|segment| {
                        // If the order is not fulfilled, we will gather the ticket id.
                        (!segment.value().is_fulfilled()).then_some(segment.key().clone())
                    })
                    // This clone is a necessary evil in order to drop the read lock;
                    // if we hold references to the orders while we await the results,
                    // the orders will be locked for the duration of an IO.
                    .collect::<Vec<_>>();

                dynamodb::get_process_successes_by_tickets::<_>(self, unfulfilled_tickets.iter())
                    .await
            }
            .await?
            .into_iter(),
        );

        for segment in self.orders().iter().await {
            let result = found_results.get(segment.key());
            if let Some(result) = result {
                segment.value().complete(*result).or_else(|err| {
                    if let CoffeeShopError::ResultAlreadySet = err {
                        crate::warn!(
                            target: LOG_TARGET,
                            "Result for ticket {} has already been set; ignoring.",
                            segment.key()
                        );
                        Ok(())
                    } else {
                        Err(err)
                    }
                })?;
            }
        }

        Ok(())
    }

    /// Periodically check DynamoDB for newly fulfilled [`Order`]s.
    ///
    /// This function will loop indefinitely until the program is terminated,
    /// or the `shutdown_signal` is triggered.
    pub async fn periodically_check_for_fulfilled_orders(
        &self,
        interval: tokio::time::Duration,
        shutdown_signal: Arc<Notify>,
    ) -> Result<(), CoffeeShopError> {
        tokio::select! {
            err = async {
                loop {
                    tokio::time::sleep(interval).await;
                    if let Err(err) = self.check_for_fulfilled_orders().await {
                        crate::error!(
                            target: LOG_TARGET,
                            "Failed to check for fulfilled orders, quitting: {}",
                            err
                        );
                        break err;
                    }
                }
            } => {
                Err(err)
            },
            _ = shutdown_signal.notified() => {
                crate::warn!(target: LOG_TARGET, "A 3rd party had requested shutdown; stop listening for SIGTERM.");
                Ok(())
            },
        }
    }
}
