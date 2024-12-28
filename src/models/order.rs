use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{errors, CoffeeShopError};

#[cfg(doc)]
use crate::models::{Barista, Shop, Waiter};

/// The log target for this module.
const LOG_TARGET: &str = "coffee_shop::models::order";

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
    pub result: tokio::sync::OnceCell<Result<O, errors::ErrorSchema>>,

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
    pub fn result(&self) -> Option<&Result<O, errors::ErrorSchema>> {
        self.result.get()
    }

    /// Consume this struct and take the result of the ticket if one is available.
    pub fn take_result(self) -> Option<Result<O, errors::ErrorSchema>> {
        self.result.into_inner()
    }

    /// Notify the waiter that the ticket is ready.
    pub fn complete(&self, result: Result<O, errors::ErrorSchema>) -> Result<(), CoffeeShopError> {
        // Set the results first, then notify the waiters.
        self.result
            .set(result)
            .map_err(|_| CoffeeShopError::ResultAlreadySet)?;
        self.notify.notify_waiters();

        Ok(())
    }

    /// Wait indefinitely for the ticket to be ready, and get the result when it is.
    ///
    /// The version of this function with a timeout is implemented as part of [`Shop`].
    pub async fn wait_until_complete(&self) -> Result<&O, CoffeeShopError> {
        // Wait until the result is set.
        loop {
            if let Some(result) = self.result() {
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
