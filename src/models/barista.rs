use serde::{de::DeserializeOwned, Serialize};
use std::{
    ops::Deref,
    sync::{atomic::AtomicUsize, Arc, Weak},
};
use tokio::sync::Notify;

use super::{
    message::{self, MulticastMessage, ProcessResult},
    Machine, Shop,
};

use crate::{helpers, models::message::MulticastMessageStatus, CoffeeShopError};

const LOG_TARGET: &str = "coffeeshop::models::barista";

/// The idle time for the barista to wait for the next ticket.
///
/// The SQS queue will be polled fresh after this duration. The SQS queue is not
/// guaranteed to be synchronised among all receivers, so it is possible that a ticket
/// sent to the queue is not immediately visible to the barista.
const BARISTA_REPORT_IDLE: tokio::time::Duration = tokio::time::Duration::from_secs(3);

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
#[derive(Debug)]
pub struct Barista<Q, I, O, F>
where
    Q: message::QueryType,
    I: Serialize + DeserializeOwned + Send + Sync,
    O: Serialize + DeserializeOwned + Send + Sync,
    F: Machine<Q, I, O>,
{
    /// A back reference to the shop that this barista is serving.
    pub shop: Weak<Shop<Q, I, O, F>>,

    /// The total amount of historical requests processed.
    pub process_count: AtomicUsize,
}

impl<Q, I, O, F> Barista<Q, I, O, F>
where
    Q: message::QueryType,
    I: Serialize + DeserializeOwned + Send + Sync,
    O: Serialize + DeserializeOwned + Send + Sync,
    F: Machine<Q, I, O>,
{
    /// Create a new [`Barista`] instance.
    pub fn new(shop: Weak<Shop<Q, I, O, F>>) -> Self {
        Self {
            shop,
            process_count: AtomicUsize::new(0),
        }
    }

    /// Get the back reference to the shop that this barista is serving.
    pub fn shop(&self) -> Arc<Shop<Q, I, O, F>> {
        self.shop.upgrade().expect("Shop has been dropped; this should not be possible in normal use. Please report this to the maintainer.")
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
    pub async fn serve(&self, shutdown_signal: Arc<Notify>) -> Result<(), CoffeeShopError> {
        let task = async {
            loop {
                crate::debug!(
                    target: LOG_TARGET,
                    "A Barista is waiting for the next ticket...",
                );
                match self.process_next_ticket(Some(BARISTA_REPORT_IDLE)).await {
                    Ok(_) => (),
                    Err(crate::CoffeeShopError::AWSSQSQueueEmpty(duration)) => crate::debug!(
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
        };

        tokio::select! {
            _ = shutdown_signal.notified() => {
                crate::warn!(
                    target: LOG_TARGET,
                    "Received shutdown signal, terminating barista."
                );

                Ok(())
            },
            _ = task => {
                unreachable!("The barista task should never return.")
            },
        }
    }

    /// Serve all the baristas in the list.
    pub async fn serve_all(
        baristas: &[Self],
        shutdown_signal: Arc<Notify>,
    ) -> Result<(), CoffeeShopError> {
        let tasks = baristas
            .iter()
            .map(|barista| barista.serve(shutdown_signal.clone()));

        futures::future::try_join_all(tasks).await.map(|_| ())
    }

    /// Process a ticket from the SQS queue.
    pub async fn process_ticket(
        &self,
        receipt: &helpers::sqs::StagedReceipt<Q, I>,
    ) -> ProcessResult<O> {
        // Increment the process count.
        self.process_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.shop()
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
        let shop = self.shop();

        // Fetch the next ticket from the SQS queue.
        let receipt: helpers::sqs::StagedReceipt<Q, I> =
            helpers::sqs::retrieve_ticket(shop.deref(), timeout).await?;

        let result = async {
            // Process the ticket.
            let process_result = self.process_ticket(&receipt).await;
            let status = if process_result.is_ok() {
                // If the processing is successful, mark the ticket as complete.
                MulticastMessageStatus::Complete
            } else {
                // If the machine failed to process it, there is not point retrying,
                // so mark it as rejected; which is different from failure.
                MulticastMessageStatus::Rejected
            };

            // Send the result to DynamoDB.
            helpers::dynamodb::put_process_result(
                shop.deref(),
                &receipt.ticket,
                process_result,
                &shop.temp_dir,
            )
            .await?;

            crate::info!(
                target: LOG_TARGET,
                "Successfully processed ticket {ticket}.",
                ticket=&receipt.ticket,
            );

            Ok::<_, CoffeeShopError>(status)
        }
        .await;

        let ticket = receipt.ticket.clone();
        let status = if let Ok(status) = result {
            status
        } else {
            MulticastMessageStatus::Failure
        };

        // TODO Send the multicast message to all the waiters.
        self.shop().announcer.send_message(
            MulticastMessage::new(
                &self.shop().name,
                &receipt.ticket,
                message::MulticastMessageKind::Ticket,
                status,
            )
        ).await.unwrap_or_else(
            |err| {
                crate::error!(
                    target: LOG_TARGET,
                    "Failed to send multicast message for ticket {ticket}, ignoring. We'll let the collection point discover the result itself: {error}",
                    ticket=&ticket,
                    error=err,
                );

                0
            }
        );

        // Delete the ticket from the queue, or put it back if the processing failed.
        if result.is_ok() {
            receipt.delete().await?;
        } else {
            // TODO stop these tickets from infinite retrying by putting them into a dead-letter queue.
            receipt.abort().await?;
        }

        result.map(|_| ())
    }
}
