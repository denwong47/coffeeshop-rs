use std::sync::Arc;

use serde::de::DeserializeOwned;

use crate::{helpers, models::Ticket, CoffeeShopError};

#[cfg(doc)]
use crate::models::{Barista, Shop, Waiter};

use super::message::ProcessResultExport;

/// The log target for this module.
const LOG_TARGET: &str = "coffeeshop::models::order";

/// A collection of [`Order`]s that are being processed.
pub type Orders = helpers::order_chain::Chain<Ticket, Order>;

/// A segment of the [`Orders`] chain.
pub type OrderSegment = helpers::order_chain::ChainSegment<Ticket, Order>;

/// A [`Delivery`] is a structure that contains:
/// - [`OnceLock`](std::sync::OnceLock) which will be populated with the processed ticket
///   once it is ready, and
/// - [`Notify`](tokio::sync::Notify) instance to notify the [`Waiter`] that the ticket is ready.
///
/// The collection point will [push the result](Delivery::complete) into the [`Delivery::result`]
/// and notify all the interested parties when the ticket is ready.
#[derive(Debug)]
pub struct Order {
    ticket: Ticket,

    /// The processed ticket result.
    pub result: std::sync::OnceLock<(tokio::time::Instant, bool)>,

    /// A [`Notify`](tokio::sync::Notify) instance to notify the waiter that the ticket is ready.
    notify: tokio::sync::Notify,
}

impl Order {
    /// Create a new [`Delivery`] instance.
    pub fn new(ticket: Ticket) -> Self {
        Self {
            ticket,
            result: std::sync::OnceLock::new(),
            notify: tokio::sync::Notify::new(),
        }
    }

    /// Get the result of the ticket if one is available.
    pub fn result(&self) -> Option<&(tokio::time::Instant, bool)> {
        self.result.get()
    }

    /// Get the age of the result.
    pub fn age_of_result(&self) -> Option<tokio::time::Duration> {
        self.result().map(|(instant, _)| instant.elapsed())
    }

    /// Complete the ticket with the result and the timestamp.
    pub fn complete_with_timestamp(
        &self,
        success: bool,
        timestamp: tokio::time::Instant,
    ) -> Result<(), CoffeeShopError> {
        // Set the results first, then notify the waiters.
        self.result
            // Add the timestamp to the result.
            .set((timestamp, success))
            .map_err(|_| CoffeeShopError::ResultAlreadySet)?;
        self.notify.notify_waiters();

        Ok(())
    }

    /// Notify the waiter that the ticket is ready.
    pub fn complete(&self, success: bool) -> Result<(), CoffeeShopError> {
        self.complete_with_timestamp(success, tokio::time::Instant::now())
    }

    /// Check if this result is fulfilled.
    pub fn is_fulfilled(&self) -> bool {
        self.result().is_some()
    }

    /// Check if this result is stale.
    ///
    /// A result is considered stale if it has a result set for more than a certain timeout,
    /// but no waiters are waiting for it.
    ///
    /// This method can only be used on [`Arc<Order>`] instances; which is typically
    /// used in conjunction with [`Orders::get`].
    pub fn is_stale(self: &Arc<Self>, max_age: std::time::Duration) -> bool {
        crate::trace!(
            target: LOG_TARGET,
            "Order has {} strong references and the result is {} seconds old.",
            Arc::strong_count(self),
            self.age_of_result().map(|age| age.as_secs_f32()).unwrap_or(0.),
        );

        matches!((Arc::strong_count(self), self.age_of_result()), (n, Some(age)) if n <= 1 && age > max_age)
    }

    /// Attempt to fetch the process result from the DynamoDB.
    pub async fn fetch<O, C>(&self, config: &C) -> Result<ProcessResultExport<O>, CoffeeShopError>
    where
        O: DeserializeOwned + Send + Sync + 'static,
        C: helpers::dynamodb::HasDynamoDBConfiguration,
    {
        helpers::dynamodb::get_process_result_by_ticket(config, &self.ticket).await
    }

    /// Wait indefinitely for the ticket to be ready, and return when it is.
    pub async fn wait_until_complete(&self) -> Result<(), CoffeeShopError> {
        // Wait until the result is set.
        loop {
            if let Some((_, status)) = self.result() {
                crate::info!(
                    target: LOG_TARGET,
                    "Ticket {ticket} is ready, status: {status}.",
                    ticket = self.ticket,
                    status = status,
                );

                return Ok(());
            } else {
                // Otherwise, wait for the notification.
                // This loop is typically only run once.
                self.notify.notified().await;
            }
        }
    }

    /// Wait for the ticket to be ready, and get the result when it is.
    ///
    /// The version of this function with a timeout is implemented as part of [`Shop`].
    pub async fn wait_and_fetch_when_complete<O, C>(
        &self,
        config: &C,
    ) -> Result<ProcessResultExport<O>, CoffeeShopError>
    where
        O: DeserializeOwned + Send + Sync + 'static,
        C: helpers::dynamodb::HasDynamoDBConfiguration,
    {
        self.wait_until_complete().await?;

        crate::info!(
            target: LOG_TARGET,
            "Ticket {ticket} is ready, fetching the result...",
            ticket = self.ticket,
        );

        // Fetch the result from the DynamoDB if there is a status available.
        // Return the result if it is set.
        self.fetch(config).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STALE_AGE: tokio::time::Duration = tokio::time::Duration::from_secs(20);

    macro_rules! create_test {
        ($name:ident(
            age=$age:literal,
            clone=$clone:literal,
            complete=$complete:literal,
            drop=$drop:literal,
            expected=$expected:literal
        )) => {
            #[tokio::test]
            async fn $name() {
                let age = tokio::time::Duration::from_secs_f32($age);
                let ticket = "test_ticket".to_owned();
                let order = Arc::new(Order::new(ticket));

                // Clone the order to increase the strong count.
                let _cloned = if $clone {
                    Some(Arc::clone(&order))
                } else {
                    None
                };

                // Complete the order.
                if $complete {
                    order
                        .complete_with_timestamp(true, tokio::time::Instant::now() - age)
                        .expect("Failed to complete the order.");
                }

                // Drop the cloned order to decrease the strong count.
                if $drop {
                    drop(_cloned);
                }

                // Check if the order is stale.
                if $expected {
                    assert!(order.is_stale(STALE_AGE));
                } else {
                    assert!(!order.is_stale(STALE_AGE));
                }
            }
        };
    }

    create_test!(new_unfulfilled_order(
        age = 0.,
        clone = false,
        complete = false,
        drop = false,
        expected = false
    ));
    create_test!(new_fulfilled_order(
        age = 0.,
        clone = false,
        complete = true,
        drop = false,
        expected = false
    ));
    create_test!(stale_fulfilled_order(
        age = 32.,
        clone = false,
        complete = true,
        drop = false,
        expected = true
    ));
    create_test!(stale_unfulfilled_order(
        age = 32.,
        clone = false,
        complete = false,
        drop = false,
        expected = false
    ));
    create_test!(stale_fulfilled_order_with_clones(
        age = 32.,
        clone = true,
        complete = true,
        drop = false,
        expected = false
    ));
    create_test!(stale_fulfilled_order_with_clones_dropped(
        age = 32.,
        clone = true,
        complete = true,
        drop = true,
        expected = true
    ));
    create_test!(stale_unfulfilled_order_with_clones(
        age = 32.,
        clone = true,
        complete = false,
        drop = false,
        expected = false
    ));
    create_test!(new_fulfilled_order_with_clones(
        age = 0.,
        clone = true,
        complete = true,
        drop = false,
        expected = false
    ));
    create_test!(new_fulfilled_order_with_clones_dropped(
        age = 0.,
        clone = true,
        complete = true,
        drop = true,
        expected = false
    ));
    create_test!(new_unfulfilled_order_with_clones(
        age = 0.,
        clone = true,
        complete = false,
        drop = false,
        expected = false
    ));
}
