use super::*;
use crate::{
    helpers::aws::{self, HasAWSSdkConfig},
    models::{
        message::{ProcessResult, ProcessResultExport},
        test::*,
        Ticket,
    },
    CoffeeMachineError, CoffeeShopError,
};
use axum::http;

const TTL: tokio::time::Duration = tokio::time::Duration::from_secs(20);
const PARTITION_KEY: &str = "identifier";

/// Convenience function to get the statics for the test.
async fn get_statics() -> (DynamoDBConfiguration, Ticket) {
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
                let (config, ticket) = get_statics().await;

                put_process_result(
                    &config,
                    &ticket,
                    $expected_result,
                )
                .await
                .expect("Failed to put the processing result into the DynamoDB table.");

                crate::info!(target: LOG_TARGET, "Put {:?} into DynamoDB table {}.", stringify!($name), config.dynamodb_table());

                // Test the fetching of statuses.
                let mut statuses = get_process_successes_by_tickets(
                    &config,
                    [ticket.clone()].iter(),
                ).await.expect("Failed to get the statuses from the DynamoDB table.");

                assert_eq!(statuses.len(), 1, "The number of statuses does not match.");

                // Test the fetching of items.
                let (actual_ticket, actual_status) = statuses.pop().unwrap();

                crate::info!(target: LOG_TARGET, "Retrieved the status from the DynamoDB table: {:?}", actual_status);
                assert_eq!(actual_ticket, ticket, "The tickets do not match.");
                assert_eq!(actual_status, $expected_result.is_ok(), "The statuses do not match.");

                let mut items = get_process_results_by_tickets::<TestResult, _>(
                    &config,
                    [ticket.clone()].iter(),
                ).await.expect("Failed to get the items from the DynamoDB table.");

                assert_eq!(items.len(), 1, "The number of items does not match.");
                let (actual_ticket, actual_result) = items.pop().unwrap();
                crate::info!(target: LOG_TARGET, "Retrieved the item from the DynamoDB table: {:?}", actual_result);

                let validate = |actual_ticket, actual_result| {
                    assert_eq!(actual_ticket, &ticket, "The tickets do not match.");
                    assert_eq!(actual_result, $expected_result.map_err(
                        |err| err.as_error_schema()
                    ), "The results differ in content.");
                };
                validate(&actual_ticket, actual_result);

                // Test the fetching of a single item.
                let actual_result = get_process_result_by_ticket::<TestResult, _>(
                    &config,
                    &ticket,
                ).await.expect("Failed to get the item from the DynamoDB table.");
                validate(&actual_ticket, actual_result);
            }
        };
    }

    create_test!(success(ProcessResult::<TestResult>::Ok(TestResult {
        greetings: "Hello, world!".to_owned(),
        narration: "A test had made a greeting.".to_owned(),
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
                let (config, expected_ticket) = get_statics().await;

                let client = dynamodb::Client::new(config.aws_config());
                crate::info!(target: LOG_TARGET, "Creating a PutItemFluentBuilder to test {result:?}...", result=$expected_result);
                let builder = client.put_item()
                    .table_name("test_table")
                    .report_ticket_result(
                        config.dynamodb_partition_key(),
                        &expected_ticket,
                        $expected_result,
                        &config.dynamodb_ttl(),
                    ).await.expect("Failed to report the ticket.");

                // We don't actually need to send the request, we can extract the item from the builder.
                let item = builder.get_item().as_ref().unwrap().clone();
                crate::info!(target: LOG_TARGET, "Extracted the item from the PutItemFluentBuilder: {:?}", item);
                drop(builder);

                let (actual_ticket, actual_result): (Ticket, ProcessResultExport<TestResult>) = item.to_process_result(config.dynamodb_partition_key())
                    .expect("Failed to convert the item to a processing result.");

                crate::info!(target: LOG_TARGET, "Converted the item of ticket {actual_ticket:?} to a processing result: {:#?}", actual_result);
                assert_eq!(actual_ticket, expected_ticket, "The tickets do not match.");
                assert_eq!(actual_result, $expected_result.map_err(
                    |err| err.as_error_schema()
                ), "The results differ in content.");
            }
        };
    }

    create_test!(success(ProcessResult::Ok(TestResult {
        greetings: "Hello, world!".to_owned(),
        narration: "A test had made a greeting.".to_owned(),
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
