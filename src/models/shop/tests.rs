//! Tests for the shop model.
//!
//! In many ways, this module behaves like an integration test, except that it aims
//! to test the inner workings of the shop without actually opening it.

use axum::http;
use std::sync::Arc;
use tokio::sync::Notify;

use crate::{models::test::*, CoffeeShopError};

const DEFAULT_TIMEOUT: tokio::time::Duration = tokio::time::Duration::from_secs(3);

mod functions_only {
    use crate::models::message::CombinedInput;

    use super::*;

    const LOG_TARGET: &str = "coffeeshop::models::order::tests::one_ticket";

    macro_rules! create_test {
        (
            $name:ident(
                query=$query:expr,
                payload=$payload:expr,
                validation_error=$validation_error:literal,
                expected=$expected:expr,
            )
        ) => {
            #[tokio::test]
            #[serial_test::serial(uses_sqs)]
            #[serial_test::serial(uses_dynamodb)]
            #[serial_test::serial(uses_multicast)]
            #[cfg(feature = "test_on_aws")]
            /// Testing the inner workings of the shop without actually opening it.
            async fn $name() {
                let shop = new_shop(1).await;

                let waiter = &shop.waiter;
                let barista = &shop.baristas.first().expect("No baristas available.");

                let query = $query;
                let payload = $payload;

                // We need some sort of mechanism to notify the barista that the waiter is dead,
                // so that it will
                // - stop waiting, and
                // - return a Ok response because its not his fault.
                let waiter_is_dead = Arc::new(Notify::new());

                // Set up the workloads that needs to be done in order to send, process and receive
                // the ticket.
                let waiter_workload = async {
                    let response = waiter
                        .create_and_retrieve_order(
                            CombinedInput::new(query, payload),
                            Some(DEFAULT_TIMEOUT),
                        )
                        .await;

                    waiter_is_dead.clone().notify_waiters();
                    Ok(response)
                };

                let barista_workload = async {
                    tokio::select! {
                        result = async {
                            barista.process_next_ticket(Some(DEFAULT_TIMEOUT)).await?;
                            shop.check_for_fulfilled_orders().await
                        } => result,
                        // If the waiter is dead, we should stop the barista, and return
                        // a Ok response.
                        _ = waiter_is_dead.notified() => {
                            crate::info!(target: LOG_TARGET, "Waiter is dead; stopping the barista.");
                            Ok(())
                        },
                    }
                };

                let response = tokio::try_join!(barista_workload, waiter_workload).expect(
                    "All of the workloads should have executed correctly, but returned an error.",
                ).1;

                crate::info!(target: LOG_TARGET, "Received response: {response:#?}");

                assert_eq!(response.status(), $expected);
                // Regardless of what happens, we should always return a JSON.
                assert_eq!(
                    response.headers().get("content-type"),
                    Some(&http::HeaderValue::from_static("application/json"))
                );
            }
        };
    }
    create_test!(good_input(
        query = TestQuery {
            name: "Big Dave".to_string(),
            timeout: Some(DEFAULT_TIMEOUT),
        },
        payload = Some(TestPayload {
            action: TestStatus::Eat,
            duration: 3600.,
        }),
        validation_error = false,
        expected = http::StatusCode::OK,
    ));

    create_test!(no_input(
        query = TestQuery {
            name: "Big Dave".to_string(),
            timeout: Some(DEFAULT_TIMEOUT),
        },
        payload = None,
        validation_error = true,
        expected = http::StatusCode::UNPROCESSABLE_ENTITY,
    ));

    create_test!(validation_error(
        query = TestQuery {
            name: "Big Dave".to_string(),
            timeout: Some(DEFAULT_TIMEOUT),
        },
        payload = Some(TestPayload {
            action: TestStatus::Eat,
            duration: -1.,
        }),
        validation_error = true,
        expected = http::StatusCode::UNPROCESSABLE_ENTITY,
    ));

    create_test!(forbidden_input(
        query = TestQuery {
            name: "Little Timmy".to_string(),
            timeout: Some(DEFAULT_TIMEOUT),
        },
        payload = Some(TestPayload {
            action: TestStatus::Eat,
            duration: 3600.,
        }),
        validation_error = false,
        expected = http::StatusCode::FORBIDDEN,
    ));

    // Same test as above, just checking that the status code is indeed
    // passed through.
    create_test!(not_acceptable_input(
        query = TestQuery {
            name: "Big Dave".to_string(),
            timeout: Some(DEFAULT_TIMEOUT),
        },
        payload = Some(TestPayload {
            action: TestStatus::Sleep,
            duration: 3600.,
        }),
        validation_error = false,
        expected = http::StatusCode::NOT_ACCEPTABLE,
    ));
}

mod announcer {
    use crate::models::message::{MulticastMessage, MulticastMessageKind, MulticastMessageStatus};

    use super::*;

    const LOG_TARGET: &str = "coffeeshop::models::shop::tests::announcer";

    #[tokio::test]
    #[serial_test::serial(uses_multicast)]
    async fn test_multicast() {
        let shop = new_shop(1).await;

        let ticket = get_random_ticket();

        let message_received = Arc::new(Notify::new());

        let sender_workload = async {
            crate::info!(target: LOG_TARGET, "Sending message to multicast channel...");
            shop.announcer
                .send_message(MulticastMessage::new(
                    &shop.name,
                    &ticket,
                    MulticastMessageKind::Ticket,
                    MulticastMessageStatus::Complete,
                ))
                .await
        };

        let receiver_workload = async {
            let start_time = tokio::time::Instant::now();
            let message_received = message_received.clone();

            crate::info!(target: LOG_TARGET, "Spawning order for ticket {}...", ticket);
            let order = shop.spawn_order(ticket.clone()).await;

            crate::info!(target: LOG_TARGET, "Waiting for ticket to be finished...");

            // Only wait for the order to complete; there is nothing to fetch.
            let result = tokio::select! {
                _ = tokio::time::sleep(DEFAULT_TIMEOUT) => {
                    crate::error!(target: LOG_TARGET, "Order for ticket {} timed out.", ticket);
                    Err(CoffeeShopError::RetrieveTimeout(start_time.elapsed()))
                },
                result = order.wait_until_complete() => {
                    result
                },
            };

            message_received.notify_waiters();

            result
        };

        let listener_workload = async {
            let message_received = message_received.clone();
            crate::info!(target: LOG_TARGET, "Listening for multicast messages...");
            shop.announcer
                .listen_for_announcements(message_received.clone())
                .await
        };

        match tokio::try_join!(listener_workload, receiver_workload, sender_workload,) {
            Ok(_) => {
                crate::info!(target: LOG_TARGET, "All workloads completed successfully.");
            }
            Err(err) => {
                crate::error!(target: LOG_TARGET, "One or more workloads failed: {err:?}");
                panic!("One or more workloads failed: {err:?}");
            }
        }
    }
}

/// Test that opens the shop.
mod open {
    use super::*;

    const LOG_TARGET: &str = "coffeeshop::models::shop::tests::open";

    #[tokio::test]
    #[serial_test::serial(uses_sqs)]
    #[serial_test::serial(uses_dynamodb)]
    #[serial_test::serial(uses_multicast)]
    #[cfg(feature = "test_on_aws")]
    async fn test_open() {
        let shop = new_shop(3).await;

        let shutdown_signal = Arc::new(Notify::new());

        let workload = async { shop.open(Some(shutdown_signal.clone())).await };

        let termination_signal = async {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            crate::info!(target: LOG_TARGET, "Sending termination signal...");
            shutdown_signal.clone().notify_waiters();

            Ok(())
        };

        let result = tokio::try_join!(workload, termination_signal,);

        match result {
            Ok(_) => {
                crate::info!(target: LOG_TARGET, "All workloads completed successfully.");
            }
            Err(err) => {
                crate::error!(target: LOG_TARGET, "One or more workloads failed: {err:?}");
                panic!("One or more workloads failed: {err:?}");
            }
        }
    }
}
