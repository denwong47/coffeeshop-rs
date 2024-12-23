use std::sync::{atomic::AtomicUsize, Arc};

use super::{Machine, Shop};

/// A [`Barista`] instance that acts as a worker for the shop.
///
/// A shop can have any positive number of [`Barista`] instances; they are responsible
/// for taking [`Ticket`]s from the SQS queue, process them, and send the results to
/// DynamoDB with the [`Ticket`] being the key.
///
/// They are also responsible for sending a multicast message to all the waiters in
/// the same cluster (including those in different [`Shop`]s), so that the waiters can
/// retrieve the results when ready instead of polling the DynamoDB table.
pub struct Barista<I, O, F>
where
    I: serde::de::DeserializeOwned + serde::Serialize,
    O: serde::Serialize + serde::de::DeserializeOwned,
    F: Machine<I, O>,
{
    /// A back reference to the shop that this barista is serving.
    pub shop: Arc<Shop<I, O, F>>,

    /// The total amount of historical requests processed.
    pub process_count: AtomicUsize,
}
