use super::*;
use crate::{helpers::aws, models::Ticket, CoffeeMachineError, CoffeeShopError};
use axum::http;

const TTL: tokio::time::Duration = tokio::time::Duration::from_secs(20);
const PARTITION_KEY: &str = "identifier";

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
async fn get_statics() -> (
    aws::SdkConfig,
    String,
    &'static str,
    Ticket,
    &'static tokio::time::Duration,
    tempfile::TempDir,
) {
    let config = aws::get_aws_config()
        .await
        .expect("Failed to get AWS configuration.");

    (
        config,
        get_dynamodb_table(),
        PARTITION_KEY,
        get_random_ticket(),
        &TTL,
        tempfile::tempdir().expect("Failed to create temporary directory."),
    )
}

mod put_items {
    use super::*;
    use serde::{Deserialize, Serialize};

    const LOG_TARGET: &str = "coffeeshop::helpers::dynamodb::tests::put_items";

    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct TestResult {
        first_name: String,
        last_name: String,
        age: u8,
    }

    macro_rules! create_test {
        ($name:ident($result:expr)) => {
            #[tokio::test]
            #[cfg(feature = "test_on_aws")]
            async fn $name() {
                let (config, table, partition_key, ticket, ttl, temp_dir) = get_statics().await;
                let result = $result;

                put_item(
                    &table,
                    partition_key,
                    &config,
                    &ticket,
                    result,
                    ttl,
                    &temp_dir,
                )
                .await
                .expect("Failed to put the processing result into the DynamoDB table.");

                crate::info!(target: LOG_TARGET, "Put {:?} into DynamoDB table {}.", stringify!($name), table);
            }
        };
    }

    create_test!(success(Ok(TestResult {
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
