use super::*;

use axum::http;
use std::sync::Arc;
use tokio::sync::Notify;

use crate::{cli::Config, helpers, models::test::*};

const STALE_AGE: tokio::time::Duration = tokio::time::Duration::from_secs(60);
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
            #[cfg(feature = "test_on_aws")]
            /// Testing the inner workings of the shop without actually opening it.
            async fn $name() {
                let shop = Shop::new(
                    LOG_TARGET.to_owned(),
                    TestMachine::new(),
                    Config::default()
                        .with_dynamodb_table(&get_dynamodb_table())
                        .with_dynamodb_partition_key("identifier")
                        .with_sqs_queue(get_queue_url())
                        .with_result_ttl(STALE_AGE.as_secs_f32()),
                    Some(
                        helpers::aws::get_aws_config()
                            .await
                            .expect("Failed to get AWS configuration."),
                    ),
                    1,
                )
                .await
                .expect("Failed to create the shop.");

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
