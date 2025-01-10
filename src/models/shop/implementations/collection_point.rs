// TODO This module needs more logging statements, and
// periodic_purge_stale_orders needs to be implemented.

use crate::{
    helpers::dynamodb::HasDynamoDBConfiguration,
    models::{message, Machine, Orders, Shop},
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
    async fn purge_stale_orders(
        &self,
        max_age: tokio::time::Duration,
    ) -> Result<(), CoffeeShopError> {
        let original_length = self.orders().len();
        self.orders().retain(|_k, v| !v.is_stale(max_age));

        crate::info!(
            target: LOG_TARGET,
            "Purged {} stale orders from the collection point.",
            original_length - self.orders().len()
        );

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
    // /// Check DynamoDB for newly fulfilled [`Order`]s.
    // ///
    // /// # Note
    // ///
    // /// Internal function: this function is not meant to be called directly.
    // pub async fn check_for_fulfilled_orders(&self) -> Result<(), CoffeeShopError> {
    //     let found_results = async {
    //         let unfulfilled_tickets = self.orders()
    //             .filter_map(|(k, v)| (!v.is_fulfilled()).then_some(k))
    //             // This clone is a necessary evil in order to drop the read lock;
    //             // if we hold references to the orders while we await the results,
    //             // the orders will be locked for the duration of an IO.
    //             .cloned()
    //             .collect::<Vec<_>>();

    //         drop(orders);

    //         dynamodb::get_process_successes_by_tickets::<_>(self, unfulfilled_tickets.iter()).await
    //     }
    //     .await?;

    //     let orders = self.orders().read().await;
    //     for (ticket, result) in found_results {
    //         if let Some(order) = orders.get(&ticket) {
    //             order.complete(result)?;
    //         }
    //     }

    //     Ok(())
    // }

    /// Periodically check DynamoDB for newly fulfilled [`Order`]s.
    ///
    /// This function will loop indefinitely until the program is terminated,
    /// or the `shutdown_signal` is triggered.
    pub async fn periodically_check_for_fulfilled_orders(
        &self,
        interval: tokio::time::Duration,
        shutdown_signal: Arc<Notify>,
    ) -> Result<(), CoffeeShopError> {
        // TODO Currently, this function is not implemented.
        crate::debug!(
            target: LOG_TARGET,
            "periodically_check_for_fulfilled_orders is not implemented; supposedly checking at {:?} seconds intervals.",
            interval.as_secs_f32()
        );
        shutdown_signal.notified().await;
        Ok(())

        // tokio::select! {
        //     err = async {
        //         loop {
        //             tokio::time::sleep(interval).await;
        //             if let Err(err) = self.check_for_fulfilled_orders().await {
        //                 crate::error!(
        //                     target: LOG_TARGET,
        //                     "Failed to check for fulfilled orders, quitting: {}",
        //                     err
        //                 );
        //                 break err;
        //             }
        //         }
        //     } => {
        //         Err(err)
        //     },
        //     _ = shutdown_signal.notified() => {
        //         crate::warn!(target: LOG_TARGET, "A 3rd party had requested shutdown; stop listening for SIGTERM.");
        //         Ok(())
        //     },
        // }
    }
}
