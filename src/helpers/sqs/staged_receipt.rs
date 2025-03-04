use aws_sdk_sqs as sqs;

use std::sync::OnceLock;

use crate::{
    helpers::{retry, serde::deserialize, sqs::HasSQSConfiguration},
    models::{message, Ticket},
    CoffeeShopError,
};

use super::encoding;

#[cfg(doc)]
use crate::models::Barista;

const LOG_TARGET: &str = "coffeeshop::helpers::sqs::staged_receipt";

/// The default wait time for receiving messages from SQS.
///
/// When there is no message in the queue, the [`Barista`]s will wait for this
/// duration before logging a message, and then checking the queue again.
const DEFAULT_WAIT_TIME: tokio::time::Duration = tokio::time::Duration::from_secs(20);

/// The maximum number of times to retry completing the message.
const MAX_COMPLETION_RETRIES: usize = 3;

/// A received message from SQS that is staged for processing, before
/// a reply to SQS had been sent on deleting the message or its visibility
/// changed back to visible.
pub struct StagedReceipt<'c, Q, I, C>
where
    Q: message::QueryType,
    I: serde::de::DeserializeOwned + serde::Serialize,
    C: HasSQSConfiguration,
{
    client: sqs::Client,
    pub ticket: Ticket,
    message: message::CombinedInput<Q, I>,
    pub receipt_handle: String,
    pub queue_url: String,

    /// Completed
    completed: OnceLock<bool>,

    config: &'c C,
}

impl<'c, Q, I, C> StagedReceipt<'c, Q, I, C>
where
    Q: message::QueryType + 'static,
    I: serde::de::DeserializeOwned + serde::Serialize + Send + Sync + 'static,
    C: HasSQSConfiguration,
{
    /// Create a new [`StagedReceipt`] instance.
    ///
    /// # Safety
    ///
    /// This method is _NOT_ cancel safe. If the future is dropped before it
    /// completes, a message could have been received, marked as invisible,
    /// and then the future is dropped, leaving the message in the queue
    /// without being processed.
    ///
    /// **Do not race this method against a timeout; use the built-in `timeout`
    /// parameter instead.**
    pub async fn receive(
        config: &'c C,
        timeout: Option<tokio::time::Duration>,
    ) -> Result<Self, CoffeeShopError> {
        let client = sqs::Client::new(config.aws_config());

        let timeout = timeout.unwrap_or(DEFAULT_WAIT_TIME).min(
            // The maximum wait time is 20 seconds, as per the AWS SQS documentation.
            tokio::time::Duration::from_secs(20),
        );

        let receive_results = client
            .receive_message()
            .queue_url(config.sqs_queue_url())
            .max_number_of_messages(1)
            .wait_time_seconds(timeout.as_secs() as i32)
            // Visibility timeout is NOT set here; we will leave it for the queue to handle.
            // .visibility_timeout(30)
            .send()
            .await
            .map_err(|sdk_err| {
                CoffeeShopError::from_aws_sqs_error(sdk_err.into_service_error().into(), config)
            })?;

        // Get one message out of the list of messages.
        // There should only be one anyway.
        let message = receive_results
            .messages
            .and_then(|mut messages| messages.pop());

        if let Some(message) = message {
            let receipt_handle = message.receipt_handle.ok_or_else(|| {
                CoffeeShopError::UnexpectedAWSResponse("Missing SQS receipt handle".to_string())
            })?;
            let body = message.body.ok_or_else(|| {
                CoffeeShopError::UnexpectedAWSResponse("Missing SQS message body".to_string())
            })?;
            let ticket = message.message_id.ok_or_else(|| {
                CoffeeShopError::UnexpectedAWSResponse("Missing SQS message ID".to_string())
            })?;

            let message =
                deserialize(encoding::decode(&body).await?)
                .inspect_err(
                    |err| {
                        if let CoffeeShopError::BinaryConversionError(_) = err {
                            #[cfg(test)]
                            crate::error!(
                                target: LOG_TARGET,
                                "Failed to deserialize the message body of ticket {} from queue {}. If this is not expected, then there could be concurrent tests interfering with each other.",
                                ticket,
                                config.sqs_queue_url(),
                            );

                            #[cfg(not(test))]
                            crate::error!(
                                target: LOG_TARGET,
                                "Failed to deserialize the message body of ticket {} from queue {}. This can be caused by Is the SQS queue exclusively used by this app?",
                                ticket,
                                config.sqs_queue_url(),
                            )
                        }
                    }
                )?;

            Ok(Self {
                client,
                ticket,
                message,
                receipt_handle,
                queue_url: config.sqs_queue_url().to_owned(),
                completed: OnceLock::new(),
                config,
            })
        } else {
            Err(CoffeeShopError::AWSSQSQueueEmpty(timeout))
        }
    }

    /// Get the query from the message.
    pub fn query(&self) -> &Q {
        &self.message.query
    }

    /// Get the input from the message.
    pub fn input(&self) -> Option<&I> {
        self.message.input.as_ref()
    }

    /// Mark the message as completed.
    pub async fn complete(self, result: bool) -> Result<(), CoffeeShopError> {
        // Check if the message has already been completed; if so, return an error.
        self.completed.set(result).map_err(|_| {
            CoffeeShopError::AWSSQSStagedReceiptAlreadyCompleted(if result {
                "deleted"
            } else {
                "aborted"
            })
        })?;

        // Create a task factory for the retry; this will be used to complete the message.
        // Each time the task failed, a new future to do the same task will be created,
        // up to a maximum of 3 times.
        let task_factory = || async {
            if result {
                crate::info!(
                    target: LOG_TARGET,
                    "Completed message processing for ticket {}, deleting it from the queue.",
                    self.ticket
                );

                // Delete the message from the queue.
                self.client
                    .delete_message()
                    .queue_url(&self.queue_url)
                    .receipt_handle(&self.receipt_handle)
                    .send()
                    .await
                    .map_err(|sdk_err| {
                        CoffeeShopError::from_aws_sqs_error(
                            sdk_err.into_service_error().into(),
                            self.config,
                        )
                    })
                    .map(|_output| ())
            } else {
                crate::warn!(
                    target: LOG_TARGET,
                    "Aborting message processing for ticket {}, returning it to the queue.",
                    self.ticket
                );

                // Change the visibility of the message back to visible.
                self.client
                    .change_message_visibility()
                    .queue_url(&self.queue_url)
                    .receipt_handle(&self.receipt_handle)
                    .visibility_timeout(0)
                    .send()
                    .await
                    .map_err(|sdk_err| {
                        CoffeeShopError::from_aws_sqs_error(
                            sdk_err.into_service_error().into(),
                            self.config,
                        )
                    })
                    .map(|_output| ())
            }
        };

        retry::until_ok("complete SQS message", task_factory, MAX_COMPLETION_RETRIES).await
    }

    /// Abort the message processing.
    pub async fn abort(self) -> Result<(), CoffeeShopError> {
        self.complete(false).await
    }

    /// Delete the message from the queue.
    pub async fn delete(self) -> Result<(), CoffeeShopError> {
        self.complete(true).await
    }
}

impl<Q, I, C> Drop for StagedReceipt<'_, Q, I, C>
where
    Q: message::QueryType,
    I: serde::de::DeserializeOwned + serde::Serialize,
    C: HasSQSConfiguration,
{
    /// Drop the [`StagedReceipt`] instance.
    ///
    /// If the message was not completed, log an error.
    /// If the `sqs_strict` feature is enabled, panic.
    ///
    /// # Note
    ///
    /// We could not use the `Drop` trait to delete the message from the queue
    /// due to the asynchronous nature of the `async fn complete` method.
    fn drop(&mut self) {
        if self.completed.get().is_none() {
            crate::error!(
                target: LOG_TARGET,
                "Staged receipt for ticket {} was dropped without being completed.",
                self.ticket
            );

            if cfg!(not(feature = "sqs_strict")) {
                panic!(
                    "Staged receipt for ticket {} was dropped without being completed; please ensure you used `StagedReceipt::delete` or `StagedReceipt::abort` to complete the message.",
                    self.ticket
                );
            }
        }
    }
}
