use crate::{
    helpers::dynamodb::{self, HasDynamoDBConfiguration},
    models::{message, Machine, Orders, Shop},
    CoffeeShopError,
};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::RwLock;

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
    fn orders(&self) -> &RwLock<Orders>;

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
    fn orders(&self) -> &RwLock<Orders> {
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
    /// Listen to the multicast messages from the [`Barista`]s.
    ///
    /// This function never returns; it will simply listen for multicast messages
    /// and spawn handlers for each received message to update the [`Order`]s.
    ///
    /// # Note
    ///
    /// Internal function: this function is not meant to be called directly.
    pub async fn listen_for_multicast(&self) -> Result<(), CoffeeShopError> {
        todo!()
    }

    /// Check DynamoDB for newly fulfilled [`Order`]s.
    ///
    /// # Note
    ///
    /// Internal function: this function is not meant to be called directly.
    pub async fn check_for_fulfilled_orders(&self) -> Result<(), CoffeeShopError> {
        let found_results = async {
            let orders = self.orders().read().await;

            let unfulfilled_tickets = orders
                .iter()
                .filter_map(|(k, v)| (!v.is_fulfilled()).then_some(k))
                // This clone is a necessary evil in order to drop the read lock;
                // if we hold references to the orders while we await the results,
                // the orders will be locked for the duration of an IO.
                .cloned()
                .collect::<Vec<_>>();

            drop(orders);

            dynamodb::get_process_successes_by_tickets::<_>(self, unfulfilled_tickets.iter()).await
        }
        .await?;

        let orders = self.orders().read().await;
        for (ticket, result) in found_results {
            if let Some(order) = orders.get(&ticket) {
                order.complete(result)?;
            }
        }

        Ok(())
    }
}
