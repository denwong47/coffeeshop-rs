use serde::{de::DeserializeOwned, Serialize};
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Weak,
};
use tokio::sync::Notify;

use super::{
    message::{self, MulticastMessage, ProcessResult},
    Machine, Shop,
};

use crate::{
    helpers::{self, sqs::HasSQSConfiguration},
    models::message::MulticastMessageStatus,
    CoffeeShopError,
};

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
    Q: message::QueryType + 'static,
    I: Serialize + DeserializeOwned + Send + Sync + 'static,
    O: Serialize + DeserializeOwned + Send + Sync + 'static,
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
    pub async fn serve(&self, is_shutdown_requested: &AtomicBool) -> Result<(), CoffeeShopError> {
        loop {
            if is_shutdown_requested.load(Ordering::Relaxed) {
                crate::warn!(
                    target: LOG_TARGET,
                    "Received shutdown signal, terminating barista."
                );

                break Ok(());
            }

            crate::trace!(
                target: LOG_TARGET,
                "A Barista is waiting for the next ticket...",
            );
            let result = self.process_next_ticket(Some(BARISTA_REPORT_IDLE)).await;

            // Inspect the result and decide what to do.
            match &result {
                Ok(_) => (),
                // Expected errors.
                Err(crate::CoffeeShopError::AWSSQSQueueEmpty(duration)) => crate::trace!(
                    target: LOG_TARGET,
                    "No tickets in the queue after {duration:?}; trying again.",
                    duration = duration,
                ),
                // Irrecoverable errors.
                Err(crate::CoffeeShopError::AWSQueueDoesNotExist(queue_url)) => {
                    crate::error!(
                        target: LOG_TARGET,
                        "The SQS queue {queue:?} does not exist; terminating barista.",
                        queue = queue_url,
                    );

                    break result;
                }
                Err(crate::CoffeeShopError::AWSDynamoDBTableDoesNotExist(table_name)) => {
                    crate::error!(
                        target: LOG_TARGET,
                        "The DynamoDB table {table:?} does not exist; terminating barista.",
                        table = table_name,
                    );

                    break result;
                }
                Err(crate::CoffeeShopError::InvalidConfiguration { field, message }) => {
                    crate::error!(
                        target: LOG_TARGET,
                        "Invalid configuration for the barista: {field}: {message}",
                        field = field,
                        message = message,
                    );

                    break result;
                }
                Err(crate::CoffeeShopError::AWSCredentialsError(err)) => {
                    crate::error!(
                        target: LOG_TARGET,
                        "AWS credentials rejected: {error}",
                        error = err,
                    );

                    break result;
                }
                // Catch all.
                Err(err) => crate::error!(
                    target: LOG_TARGET,
                    "Error processing ticket: {error}",
                    error = err,
                ),
            }
        }
    }

    /// Serve all the baristas in the list.
    pub async fn serve_all(
        baristas: &[Self],
        shutdown_signal: Arc<Notify>,
    ) -> Result<(), CoffeeShopError> {
        let is_shutdown_requested = AtomicBool::new(false);

        let shutdown_signal_task = async {
            shutdown_signal.notified().await;
            crate::warn!(
                target: LOG_TARGET,
                "Received shutdown signal, flagging all baristas to terminate after current workload."
            );
            is_shutdown_requested.store(true, Ordering::Relaxed);

            Ok(())
        };

        let tasks = baristas
            .iter()
            .map(|barista| barista.serve(&is_shutdown_requested));

        tokio::try_join!(shutdown_signal_task, futures::future::try_join_all(tasks),).map(|_| ())
    }

    /// Process a ticket from the SQS queue.
    pub async fn process_ticket<C>(
        &self,
        receipt: &helpers::sqs::StagedReceipt<'_, Q, I, C>,
    ) -> ProcessResult<O>
    where
        C: HasSQSConfiguration,
    {
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
        let receipt: helpers::sqs::StagedReceipt<'_, Q, I, _> =
            helpers::sqs::retrieve_ticket(&shop, timeout).await?;

        let result = async {
            // Process the ticket.
            let process_result = self.process_ticket(&receipt).await;
            let status = if process_result.is_ok() {
                // If the processing is successful, mark the ticket as complete.
                MulticastMessageStatus::Success
            } else {
                // If the machine failed to process it, there is not point retrying,
                // so mark it as rejected; which is different from failure.
                MulticastMessageStatus::Aborted
            };

            // Send the result to DynamoDB.
            helpers::dynamodb::put_process_result(&shop, &receipt.ticket, process_result).await?;

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
            MulticastMessageStatus::Error
        };

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
        let is_ok = result.is_ok();

        if is_ok {
            receipt.delete().await
        } else {
            // TODO stop these tickets from infinite retrying by putting them into a dead-letter queue.
            receipt.abort().await
        }.unwrap_or_else(
            |err| {
                crate::error!(
                    target: LOG_TARGET,
                    "Failed to {action} ticket {ticket}, ignoring. This ticket may get executed again: {error:?}",
                    action=if is_ok { "delete" } else { "abort" },
                    ticket=&ticket,
                    error=err,
                );
            }
        );

        result.map(|_| ())
    }
}
