use super::{message, Machine, Orders, Shop};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::RwLock;

#[cfg(doc)]
use super::{Barista, Waiter};

/// A [`CollectionPoint`] is a behaviour of a [`Shop`] that:
/// - Monitors the orders on DynamoDB that is flagged by the [`Waiter`]s
/// - Listens for Multicast messages from the [`Barista`]s from this and other [`Shop`]s
/// - Update the [`Order`]s in the [`Shop`] instance with the results from the [`Barista`]s
#[async_trait::async_trait]
pub trait CollectionPoint<O>
where
    O: Serialize + DeserializeOwned,
{
    /// Access the orders relevant to the collection point.
    fn orders(&self) -> &RwLock<Orders<O>>;

    // TODO Complete this trait
}

#[async_trait::async_trait]
impl<Q, I, O, F> CollectionPoint<O> for Shop<Q, I, O, F>
where
    Q: message::QueryType,
    I: Serialize + DeserializeOwned,
    O: Serialize + DeserializeOwned,
    F: Machine<Q, I, O>,
{
    /// Access the orders in the [`Shop`] instance.
    fn orders(&self) -> &RwLock<Orders<O>> {
        &self.orders
    }
}
