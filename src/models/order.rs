use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use hashbrown::HashMap;

use crate::{errors, CoffeeShopError};

#[cfg(doc)]
use crate::models::{Barista, Shop, Waiter};

/// The log target for this module.
const LOG_TARGET: &str = "coffee_shop::models::order";

/// A collection of [`Order`]s that are being processed.
pub type Orders<O> = HashMap<String, Arc<Order<O>>>;

/// A [`Delivery`] is a structure that contains:
/// - [`OnceCell`](tokio::sync::OnceCell) which will be populated with the processed ticket
///   once it is ready, and
/// - [`Notify`](tokio::sync::Notify) instance to notify the [`Waiter`] that the ticket is ready.
///
/// The collection point will [push the result](Delivery::complete) into the [`Delivery::result`]
/// and notify all the interested parties when the ticket is ready.
#[derive(Debug)]
pub struct Order<O>
where
    O: serde::Serialize + serde::de::DeserializeOwned,
{
    /// The processed ticket result.
    pub result: tokio::sync::OnceCell<(tokio::time::Instant, Result<O, errors::ErrorSchema>)>,

    /// A [`Notify`](tokio::sync::Notify) instance to notify the waiter that the ticket is ready.
    notify: tokio::sync::Notify,

    /// A flag to indicate if the order had been fulfilled with no further
    /// waiters waiting for the result.
    ///
    /// This is not considered accurate because the waiter could have been
    /// Dropped as part of [`tokio::select`] while waiting for the result.
    waiters_count: AtomicUsize,
}

impl<O> Default for Order<O>
where
    O: serde::Serialize + serde::de::DeserializeOwned,
{
    fn default() -> Self {
        Self {
            result: tokio::sync::OnceCell::new(),
            notify: tokio::sync::Notify::new(),
            waiters_count: AtomicUsize::new(0),
        }
    }
}

impl<O> Order<O>
where
    O: serde::Serialize + serde::de::DeserializeOwned,
{
    /// Create a new [`Delivery`] instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Get the result of the ticket if one is available.
    pub fn result(&self) -> Option<&(tokio::time::Instant, Result<O, errors::ErrorSchema>)> {
        self.result.get()
    }

    /// Get the age of the result.
    pub fn age_of_result(&self) -> Option<tokio::time::Duration> {
        self.result().map(|(instant, _)| instant.elapsed())
    }

    /// Complete the ticket with the result and the timestamp.
    pub fn complete_with_timestamp(
        &self,
        result: Result<O, errors::ErrorSchema>,
        timestamp: tokio::time::Instant,
    ) -> Result<(), CoffeeShopError> {
        // Set the results first, then notify the waiters.
        self.result
            // Add the timestamp to the result.
            .set((timestamp, result))
            .map_err(|_| CoffeeShopError::ResultAlreadySet)?;
        self.notify.notify_waiters();

        Ok(())
    }

    /// Notify the waiter that the ticket is ready.
    pub fn complete(&self, result: Result<O, errors::ErrorSchema>) -> Result<(), CoffeeShopError> {
        self.complete_with_timestamp(result, tokio::time::Instant::now())
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

    /// Wait indefinitely for the ticket to be ready, and get the result when it is.
    ///
    /// The version of this function with a timeout is implemented as part of [`Shop`].
    pub async fn wait_until_complete(&self) -> Result<&O, CoffeeShopError> {
        // Wait until the result is set.
        loop {
            if let Some((_, result)) = self.result() {
                let waiters_count = self
                    .waiters_count
                    .fetch_sub(1, Ordering::Relaxed)
                    .saturating_sub(1);

                crate::info!(
                    target: LOG_TARGET,
                    "Order is ready with {} waiters still waiting for the results.",
                    waiters_count
                );

                // Return the result if it is set.
                return result
                    .as_ref()
                    .map_err(|err| CoffeeShopError::ErrorSchema(err.clone()));
            } else {
                self.waiters_count.fetch_add(1, Ordering::Relaxed);

                // Otherwise, wait for the notification.
                // This loop is typically only run once.
                self.notify.notified().await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STALE_AGE: tokio::time::Duration = tokio::time::Duration::from_secs(30);

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
                let order = Arc::new(Order::<()>::new());

                // Clone the order to increase the strong count.
                let _cloned = if $clone {
                    Some(Arc::clone(&order))
                } else {
                    None
                };

                // Complete the order.
                if $complete {
                    order
                        .complete_with_timestamp(Ok(()), tokio::time::Instant::now() - age)
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
