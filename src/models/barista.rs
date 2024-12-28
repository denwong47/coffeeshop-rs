use std::{
    ops::Deref,
    sync::{atomic::AtomicUsize, Arc},
};

use super::{message, Machine, Shop};

use crate::{helpers, CoffeeShopError};

const LOG_TARGET: &str = "coffeeshop::models::barista";

#[cfg(doc)]
use crate::models::Ticket;

/// A [`Barista`] instance that acts as a worker for the shop.
///
/// A shop can have any positive number of [`Barista`] instances; they are responsible
/// for taking [`Ticket`]s from the SQS queue, process them, and send the results to
/// DynamoDB with the [`Ticket`] being the key.
///
/// They are also responsible for sending a multicast message to all the waiters in
/// the same cluster (including those in different [`Shop`]s), so that the waiters can
/// retrieve the results when ready instead of polling the DynamoDB table.
pub struct Barista<Q, I, O, F>
where
    Q: message::QueryType,
    I: serde::de::DeserializeOwned + serde::Serialize,
    O: serde::Serialize + serde::de::DeserializeOwned,
    F: Machine<Q, I, O>,
{
    /// A back reference to the shop that this barista is serving.
    pub shop: Arc<Shop<Q, I, O, F>>,

    /// The total amount of historical requests processed.
    pub process_count: AtomicUsize,
}

impl<Q, I, O, F> Barista<Q, I, O, F>
where
    Q: message::QueryType,
    I: serde::de::DeserializeOwned + serde::Serialize,
    O: serde::Serialize + serde::de::DeserializeOwned + Send + Sync,
    F: Machine<Q, I, O>,
{
    /// Create a new [`Barista`] instance.
    pub fn new(shop: Arc<Shop<Q, I, O, F>>) -> Self {
        Self {
            shop,
            process_count: AtomicUsize::new(0),
        }
    }

    /// Get the total amount of historical requests processed.
    pub fn get_process_count(&self) -> usize {
        self.process_count
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Ask the [`Barista`] to start serving.
    ///
    /// This function never returns, and will loop indefinitely until the
    /// program is terminated.
    pub async fn serve(&self, timeout: Option<tokio::time::Duration>) {
        loop {
            crate::info!(
                target: LOG_TARGET,
                "A Barista is waiting for the next ticket...",
            );
            match self.process_next_ticket(timeout).await {
                Ok(_) => (),
                Err(crate::CoffeeShopError::AWSSQSQueueEmpty(duration)) => crate::info!(
                    target: LOG_TARGET,
                    "No tickets in the queue after {duration:?}; trying again.",
                    duration = duration,
                ),
                Err(err) => crate::error!(
                    target: LOG_TARGET,
                    "Error processing ticket: {error}",
                    error = err,
                ),
            }
        }
    }

    /// Process a ticket from the SQS queue.
    pub async fn process_ticket(
        &self,
        receipt: &helpers::sqs::StagedReceipt<Q, I>,
    ) -> Result<O, crate::CoffeeShopError> {
        // Increment the process count.
        self.process_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.shop
            .coffee_machine
            .call(receipt.query(), receipt.input())
            .await
            .map_err(CoffeeShopError::ProcessingError)
    }

    /// Fetch the next ticket from the SQS queue, process it, and send the result to DynamoDB.
    #[allow(unused_variables)]
    pub async fn process_next_ticket(
        &self,
        timeout: Option<tokio::time::Duration>,
    ) -> Result<(), crate::CoffeeShopError> {
        // Fetch the next ticket from the SQS queue.
        let receipt: helpers::sqs::StagedReceipt<Q, I> =
            helpers::sqs::retrieve_ticket(self.shop.deref(), timeout).await?;

        // Process the ticket.
        let process_result = self.process_ticket(&receipt).await;

        // Send the result to DynamoDB.
        helpers::dynamodb::put_item(
            self.shop.deref(),
            &receipt.ticket,
            process_result,
            &self.shop.temp_dir,
        )
        .await?;

        crate::info!(
            target: LOG_TARGET,
            "Successfully processed ticket {ticket}.",
            ticket=&receipt.ticket,
        );

        // Send the multicast message to all the waiters.
        todo!()
    }
}
