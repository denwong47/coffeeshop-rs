use super::*;
use crate::{
    helpers::aws,
    models::{message, test::*},
    CoffeeShopError,
};

/// Test if the `put_ticket` function works as expected.
///
/// Since queue purging can only be done once every 60 seconds, the tests in this
/// module are run serially with only the first test purging the queue.
///
/// **They cannot be re-ordered.**
mod full_workflow {
    use super::*;

    const LOG_TARGET: &str = "coffeeshop::helpers::sqs::tests::full_workflow";

    const TIMEOUT: Option<tokio::time::Duration> = Some(tokio::time::Duration::from_secs(20));

    /// Convenience function to get the statics for the test.
    async fn get_statics() -> SQSConfiguration {
        let config = aws::get_aws_config()
            .await
            .expect("Failed to get AWS configuration.");

        let queue_url = get_queue_url();

        SQSConfiguration {
            queue_url,
            aws_config: config,
        }
    }

    #[serial_test::serial(uses_sqs)]
    #[tokio::test]
    #[cfg(feature = "test_on_aws")]
    async fn get_from_empty_queue() {
        let config = get_statics().await;

        let ticket_count = get_ticket_count(&config)
            .await
            .expect("Failed to get the ticket count.");
        // Purge the queue. This is necessary because the queue may not be empty.
        if ticket_count > 0 {
            crate::info!(target: LOG_TARGET, "Ticket count is {}, purging tickets from {}...", ticket_count, config.sqs_queue_url());
            purge_tickets(&config)
                .await
                .expect("Failed to purge the queue.");
        } else {
            crate::info!(target: LOG_TARGET, "Queue is already empty, no need to purge.");
        }

        crate::debug!(target: LOG_TARGET, "Retrieving ticket from the empty queue of {}...", config.sqs_queue_url());
        let has_timedout = tokio::select! {
            result = retrieve_ticket::<TestQuery, TestPayload, _>(&config, Some(tokio::time::Duration::from_secs(1))) => {
                match result {
                    Ok(receipt) => {
                        crate::warn!(target: LOG_TARGET, "Received unexpected ticket! Are there other concurrent tests interfering with this one?");
                        receipt.delete().await.expect("Failed to delete the unexpected ticket.");

                        false
                    },
                    // This is the desired outcome: the queue is empty, and we are being timed out
                    // by AWS instead of locally.
                    Err(CoffeeShopError::AWSSQSQueueEmpty(_)) => true,
                    Err(err) => {
                        crate::warn!(target: LOG_TARGET, "Unexpected failure while waiting for empty queue: {:?}", err);
                        false
                    }
                }
            },
            // This duration MUST be longer than the timeout in the above query.
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(2)) => {
                crate::error!(target: LOG_TARGET, "Locally timed out while waiting for empty queue.");
                false
            },
        };

        assert!(
            has_timedout,
            "Did not time out while waiting for empty queue. Use `RUST_LOG=info` to see the logs."
        );
    }

    #[serial_test::serial(uses_sqs)]
    #[tokio::test]
    #[cfg(feature = "test_on_aws")]
    async fn put_and_delete_ticket() {
        let config = get_statics().await;

        // Building the queries and payloads.
        let query = TestQuery {
            name: "big dave".to_string(),
            // This is actually a different timeout from the `timeout` variable, but it's just for testing.
            timeout: TIMEOUT,
            is_async: false,
        };

        let payload = TestPayload {
            action: TestStatus::Eat,
            duration: 1.0,
        };

        let queue_url = config.sqs_queue_url();

        crate::debug!(target: LOG_TARGET, "Putting ticket into {}...", queue_url);

        // Put the ticket into the queue.
        let ticket = put_ticket(
            &config,
            message::CombinedInput::new(query.clone(), Some(payload.clone())),
        )
        .await
        .expect("Failed to put the ticket into the queue.");

        crate::info!(target: LOG_TARGET, "Got ticket #{}.", &ticket);
        crate::debug!(target: LOG_TARGET, "Retrieving ticket from {}...", queue_url);

        let receipt: StagedReceipt<TestQuery, TestPayload, _> = retrieve_ticket(&config, TIMEOUT)
            .await
            .expect("Failed to retrieve the ticket from the queue.");

        assert_eq!(&receipt.ticket, &ticket);

        assert_eq!(receipt.query(), &query);
        assert_eq!(receipt.input(), Some(&payload));

        receipt
            .delete()
            .await
            .expect("Failed to finish the receipt.");

        crate::info!(target: LOG_TARGET, "Deleted ticket #{}.", &ticket);

        match retrieve_ticket::<TestQuery, TestPayload, _>(
            &config,
            Some(tokio::time::Duration::from_secs(1)),
        )
        .await
        {
            Ok(receipt) => {
                receipt
                    .delete()
                    .await
                    .expect("Failed to delete the unexpected ticket.");
                panic!("Queue was not empty after deletion.");
            }
            Err(CoffeeShopError::AWSSQSQueueEmpty(_)) => {
                crate::info!(target: LOG_TARGET, "Queue is empty after deletion.");
            }
            Err(err) => {
                panic!("Unexpected error while waiting for empty queue: {:?}", err);
            }
        };
    }

    #[serial_test::serial(uses_sqs)]
    #[tokio::test]
    #[cfg(feature = "test_on_aws")]
    async fn put_and_abort_ticket() {
        let config = get_statics().await;
        let queue_url = config.sqs_queue_url();

        // Building the queries and payloads.
        let query = TestQuery {
            name: "big dave".to_string(),
            // This is actually a different timeout from the `timeout` variable, but it's just for testing.
            timeout: TIMEOUT,
            is_async: false,
        };

        let payload = TestPayload {
            action: TestStatus::Work,
            duration: 42.0,
        };

        crate::debug!(target: LOG_TARGET, "Putting ticket into {}...", queue_url);

        // Put the ticket into the queue.
        let ticket = put_ticket(
            &config,
            message::CombinedInput::new(query.clone(), Some(payload.clone())),
        )
        .await
        .expect("Failed to put the ticket into the queue.");

        crate::info!(target: LOG_TARGET, "Got ticket #{}.", &ticket);
        crate::debug!(target: LOG_TARGET, "Retrieving ticket from {}...", queue_url);

        let receipt: StagedReceipt<TestQuery, TestPayload, _> = retrieve_ticket(&config, TIMEOUT)
            .await
            .expect("Failed to retrieve the ticket from the queue.");

        assert_eq!(&receipt.ticket, &ticket);

        assert_eq!(receipt.query(), &query);
        assert_eq!(receipt.input(), Some(&payload));

        receipt.abort().await.expect("Failed to abort the receipt.");

        crate::info!(target: LOG_TARGET, "Aborted ticket #{}. Trying again to retrieve it...", &ticket);

        match retrieve_ticket::<TestQuery, TestPayload, _>(
            &config,
            Some(tokio::time::Duration::from_secs(1)),
        )
        .await
        {
            Ok(receipt) => {
                assert_eq!(&receipt.ticket, &ticket, "Ticket mismatch after aborting.");

                assert_eq!(receipt.query(), &query);
                assert_eq!(receipt.input(), Some(&payload));

                receipt
                    .delete()
                    .await
                    .expect("Failed to delete the unexpected ticket.");
            }
            Err(CoffeeShopError::AWSSQSQueueEmpty(_)) => {
                panic!("Queue is empty after abortion.");
            }
            Err(err) => {
                panic!("Unexpected error while waiting for empty queue: {:?}", err);
            }
        };
    }
}
