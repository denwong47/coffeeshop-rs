//! [`Order`] related methods that live in the [`Shop`] struct.
//!

use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

use super::Shop;
use crate::{
    helpers,
    models::{
        message,
        order::{Order, OrderSegment},
        Machine, Ticket,
    },
};

const LOG_TARGET: &str = "coffeeshop::models::shop::order";

impl<Q, I, O, F> Shop<Q, I, O, F>
where
    Q: message::QueryType + 'static,
    I: Serialize + DeserializeOwned + Send + Sync + 'static,
    O: Serialize + DeserializeOwned + Send + Sync + 'static,
    F: Machine<Q, I, O>,
{
    /// Check if this shop has an order for a given ticket.
    ///
    /// # Note
    ///
    /// If you intend to use the order after this function, use [`Shop::get_order`] instead.
    pub async fn has_order(&self, ticket: &Ticket) -> bool {
        self.orders.contains_key(ticket).await
    }

    /// Get the order for a given ticket in the shop.
    pub async fn get_order(&self, ticket: &Ticket) -> Option<Arc<OrderSegment>> {
        self.orders.get(ticket).await
    }

    /// Spawn a [`Order`] order for a given [`Ticket`] in the shop.
    ///
    /// Get the ticket if it exists, otherwise create a new one
    /// before returning the [`Arc`] reference to the [`Order`].
    pub async fn spawn_order(&self, ticket: Ticket) -> Arc<OrderSegment> {
        #[cfg(feature = "debug")]
        let start_time = tokio::time::Instant::now();

        let result = self.orders.insert(ticket.clone(), Order::new(ticket)).await;

        crate::debug!(
            target: LOG_TARGET,
            "Spawned order in {:?}.",
            start_time.elapsed()
        );

        match result {
            Ok(segment) => segment,
            Err(helpers::order_chain::AttachmentError::KeyAlreadyExists { existing, .. }) => {
                existing
            }
            Err(err) => unreachable!(
                "No other error should be possible from `ChainSegment::insert`: {:?}",
                err
            ),
        }
    }
}
