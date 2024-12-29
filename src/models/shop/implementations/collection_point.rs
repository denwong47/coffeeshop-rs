use crate::{
    helpers::dynamodb::HasDynamoDBConfiguration,
    models::{message, Machine, Order, Orders, Shop, Ticket},
    CoffeeShopError,
};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg(doc)]
use crate::models::{Barista, Waiter};

const LOG_TARGET: &str = "coffee_shop::models::collection_point";

/// A [`CollectionPoint`] is a behaviour of a [`Shop`] that:
/// - Monitors the orders on DynamoDB that is flagged by the [`Waiter`]s
/// - Listens for Multicast messages from the [`Barista`]s from this and other [`Shop`]s
/// - Update the [`Order`]s in the [`Shop`] instance with the results from the [`Barista`]s
#[async_trait::async_trait]
pub trait CollectionPoint<O>: HasDynamoDBConfiguration
where
    O: Serialize + DeserializeOwned + Send + Sync,
{
    /// Access the orders relevant to the collection point.
    fn orders(&self) -> &RwLock<Orders<O>>;

    /// Purge stale orders from the collection point.
    ///
    /// Currently, this function never fails; the error type is reserved for future use.
    async fn purge_stale_orders(
        &self,
        max_age: tokio::time::Duration,
    ) -> Result<(), CoffeeShopError> {
        let mut orders = self.orders().write().await;

        let removed = orders.extract_if(|_k, v| v.is_stale(max_age));

        crate::info!(
            target: LOG_TARGET,
            "Purged {} stale orders from the collection point.",
            removed.count()
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

    /// Get the unfulfilled orders from the collection point.
    async fn unfulfilled_orders(&self) -> Vec<(Ticket, Arc<Order<O>>)> {
        let orders = self.orders().read().await;

        orders
            .iter()
            .filter(|&(_, v)| (!v.is_fulfilled()))
            .map(|(k, v)| (k.to_owned(), Arc::clone(v)))
            .collect()
    }

    /// Listen to the multicast messages from the [`Barista`]s.
    ///
    /// This function never returns; it will simply listen for multicast messages
    /// and spawn handlers for each received message to update the [`Order`]s.
    async fn listen_for_multicast(&self) -> Result<(), CoffeeShopError> {
        todo!()
    }

    /// Check DynamoDB for newly fulfilled [`Order`]s.
    async fn check_for_fulfilled_orders(&self) -> Result<(), CoffeeShopError> {
        todo!()
    }
}

#[async_trait::async_trait]
impl<Q, I, O, F> CollectionPoint<O> for Shop<Q, I, O, F>
where
    Q: message::QueryType,
    I: Serialize + DeserializeOwned,
    O: Serialize + DeserializeOwned + Send + Sync,
    F: Machine<Q, I, O>,
{
    /// Access the orders in the [`Shop`] instance.
    fn orders(&self) -> &RwLock<Orders<O>> {
        &self.orders
    }
}
