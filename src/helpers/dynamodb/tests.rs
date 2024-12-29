use super::*;
use crate::{
    helpers::aws::{self, HasAWSSdkConfig},
    models::{message::ProcessResult, Ticket},
    CoffeeMachineError, CoffeeShopError,
};
use axum::http;
use serde::{Deserialize, Serialize};

const TTL: tokio::time::Duration = tokio::time::Duration::from_secs(20);
const PARTITION_KEY: &str = "identifier";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct TestResult {
    first_name: String,
    last_name: String,
    age: u8,
}

/// Get the queue URL from the environment variables.
///
/// In order for this test to run, the environment variable `TEST_QUEUE_URL` must be set to
/// the URL of the queue to test on.
///
/// # Warning
///
/// The queue will be purged multiple times during the test, so make sure that the queue is
/// not used for other purposes.
fn get_dynamodb_table() -> String {
    std::env::var("TEST_DYNAMODB_TABLE")
        .expect("TEST_DYNAMODB_TABLE not set; please set it in the environment variables.")
}

/// Generate a random [`Ticket`].
///
/// This is useful because these tests do not actually involve SQS, which is normally where
/// the tickets are generated.
fn get_random_ticket() -> Ticket {
    uuid::Uuid::new_v4().to_string()
}

/// Convenience function to get the statics for the test.
async fn get_statics() -> (DynamoDBConfiguration, Ticket, tempfile::TempDir) {
    let config = aws::get_aws_config()
        .await
        .expect("Failed to get AWS configuration.");

    (
        DynamoDBConfiguration {
            table: get_dynamodb_table(),
            partition_key: PARTITION_KEY.to_owned(),
            ttl: TTL,
            aws_config: config,
        },
        get_random_ticket(),
        tempfile::tempdir().expect("Failed to create temporary directory."),
    )
}

mod put_items {
    use super::*;

    const LOG_TARGET: &str = "coffeeshop::helpers::dynamodb::tests::put_items";

    macro_rules! create_test {
        ($name:ident($expected_result:expr)) => {
            #[tokio::test]
            #[cfg(feature = "test_on_aws")]
            async fn $name() {
                let (config, ticket, temp_dir) = get_statics().await;

                put_item(
                    &config,
                    &ticket,
                    $expected_result,
                    &temp_dir,
                )
                .await
                .expect("Failed to put the processing result into the DynamoDB table.");

                crate::info!(target: LOG_TARGET, "Put {:?} into DynamoDB table {}.", stringify!($name), config.dynamodb_table());

                let mut items = get_items_by_tickets::<TestResult>(
                    &config,
                    [ticket.clone()].iter(),
                ).await.expect("Failed to get the items from the DynamoDB table.");

                assert_eq!(items.len(), 1, "The number of items does not match.");
                let (actual_ticket, actual_result) = items.pop().unwrap();
                crate::info!(target: LOG_TARGET, "Retrieved the item from the DynamoDB table: {:?}", actual_result);

                assert_eq!(actual_ticket, ticket, "The tickets do not match.");
                assert_eq!(actual_result, $expected_result, "The results differ in content.");
            }
        };
    }

    create_test!(success(Ok::<_, CoffeeShopError>(TestResult {
        first_name: "Big".to_string(),
        last_name: "Dave".to_string(),
        age: 42,
    })));

    create_test!(failure_host(Err::<TestResult, _>(
        CoffeeShopError::UnexpectedAWSResponse("Test error message.".to_string(),)
    )));

    create_test!(failure_process(Err::<TestResult, _>(
        CoffeeShopError::ProcessingError(CoffeeMachineError::new(
            http::StatusCode::IM_A_TEAPOT,
            "ImATeaPot".to_owned(),
            Some(serde_json::json!({
                "message": "The HTTP 418 I'm a teapot status response code indicates that the server refuses to brew coffee because it is, permanently, a teapot.",
                "notes": [
                    "A combined coffee/tea pot that is temporarily out of coffee should instead return 503.",
                    "This error is a reference to Hyper Text Coffee Pot Control Protocol defined in April Fools' jokes in 1998 and 2014."
                ]
            }))
        ))
    )));
}

mod process_result_to_and_from_item {
    use super::*;

    use aws_sdk_dynamodb as dynamodb;

    const LOG_TARGET: &str =
        "coffeeshop::helpers::dynamodb::tests::process_result_to_and_from_item";

    macro_rules! create_test {
        (
            $name:ident($expected_result:expr)
        ) => {
            #[tokio::test]
            async fn $name() {
                let (config, expected_ticket, temp_dir) = get_statics().await;

                let client = dynamodb::Client::new(config.aws_config());
                crate::info!(target: LOG_TARGET, "Creating a PutItemFluentBuilder to test {result:?}...", result=$expected_result);
                let builder = client.put_item()
                    .table_name("test_table")
                    .report_ticket_result(
                        config.dynamodb_partition_key(),
                        &expected_ticket,
                        $expected_result,
                        &config.dynamodb_ttl(),
                        &temp_dir,
                    ).await.expect("Failed to report the ticket.");

                // We don't actually need to send the request, we can extract the item from the builder.
                let item = builder.get_item().as_ref().unwrap().clone();
                crate::info!(target: LOG_TARGET, "Extracted the item from the PutItemFluentBuilder: {:?}", item);
                drop(builder);

                let (actual_ticket, actual_result): (Ticket, ProcessResult<TestResult>) = item.to_process_result(config.dynamodb_partition_key())
                    .expect("Failed to convert the item to a processing result.");

                crate::info!(target: LOG_TARGET, "Converted the item of ticket {actual_ticket:?} to a processing result: {:#?}", actual_result);
                assert_eq!(actual_ticket, expected_ticket, "The tickets do not match.");
                assert_eq!(actual_result, $expected_result, "The results differ in content.");
            }
        };
    }

    create_test!(success(ProcessResult::Ok(TestResult {
        first_name: "Big".to_string(),
        last_name: "Dave".to_string(),
        age: 42,
    })));

    create_test!(failure_host(ProcessResult::<TestResult>::Err(
        CoffeeShopError::UnexpectedAWSResponse("Test error message.".to_string(),)
    )));

    create_test!(failure_process(ProcessResult::<TestResult>::Err(
        CoffeeShopError::ProcessingError(CoffeeMachineError::new(
            http::StatusCode::IM_A_TEAPOT,
            "ImATeaPot".to_owned(),
            Some(serde_json::json!({
                "message": "The HTTP 418 I'm a teapot status response code indicates that the server refuses to brew coffee because it is, permanently, a teapot.",
                "notes": [
                    "A combined coffee/tea pot that is temporarily out of coffee should instead return 503.",
                    "This error is a reference to Hyper Text Coffee Pot Control Protocol defined in April Fools' jokes in 1998 and 2014."
                ]
            }))
        ))
    )));
}
