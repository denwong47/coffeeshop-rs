//! Test models for testing the message module.
//!
use std::sync::Arc;

use crate::{
    cli::Config,
    helpers,
    models::{message, Machine, Shop, Ticket},
    CoffeeMachineError, ValidationError,
};
use axum::http;
use serde::{Deserialize, Serialize};

const LOG_TARGET: &str = "coffeeshop::models::test";

/// The default time to live for the results in the DynamoDB table.
pub const STALE_AGE: tokio::time::Duration = tokio::time::Duration::from_secs(60);

/// Get the queue URL from the environment variables.
///
/// In order for this test to run, the environment variable `TEST_QUEUE_URL` must be set to
/// the URL of the queue to test on.
///
/// # Warning
///
/// The queue will be purged multiple times during the test, so make sure that the queue is
/// not used for other purposes.
pub fn get_queue_url() -> String {
    std::env::var("TEST_QUEUE_URL")
        .expect("TEST_QUEUE_URL not set; please set it in the environment variables.")
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
pub fn get_dynamodb_table() -> String {
    std::env::var("TEST_DYNAMODB_TABLE")
        .expect("TEST_DYNAMODB_TABLE not set; please set it in the environment variables.")
}

/// Generate a random [`Ticket`].
///
/// This is useful because these tests do not actually involve SQS, which is normally where
/// the tickets are generated.
pub fn get_random_ticket() -> Ticket {
    uuid::Uuid::new_v4().to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum TestStatus {
    Eat,
    Sleep,
    Work,
}

impl Default for TestStatus {
    fn default() -> Self {
        Self::Eat
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TestQuery {
    pub name: String,
    pub timeout: Option<tokio::time::Duration>,
}

impl message::QueryType for TestQuery {
    fn get_timeout(&self) -> Option<tokio::time::Duration> {
        self.timeout
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TestPayload {
    pub action: TestStatus,
    pub duration: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TestResult {
    pub greetings: String,
    pub narration: String,
}

pub struct TestMachine {}

impl TestMachine {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl Machine<TestQuery, TestPayload, TestResult> for TestMachine {
    async fn call(
        &self,
        query: &TestQuery,
        input: Option<&TestPayload>,
    ) -> message::MachineResult<TestResult> {
        self.validate(query, input).await?;

        let payload = input.unwrap();

        // Simulate runtime errors.
        // Assume these are not possible to validate.
        if payload.action == TestStatus::Sleep {
            // This is a contrived example to show how to return an error.
            return Err(CoffeeMachineError::new(
                http::StatusCode::NOT_ACCEPTABLE,
                "NoSleepForYou".to_owned(),
                Some(serde_json::json!({
                    "message": format!(
                        "{name} is not allowed to sleep.",
                        name = query.name,
                    ),
                })),
            ));
        } else if &query.name == "Little Timmy" {
            return Err(CoffeeMachineError::new(
                http::StatusCode::FORBIDDEN,
                "NoTimmy".to_owned(),
                Some(serde_json::json!({
                    "message": "Little Timmy is not allowed in the coffee shop.",
                })),
            ));
        }

        Ok(TestResult {
            greetings: format!("Hello, {name}!", name = query.name),
            narration: format!(
                "You want to {action:?} for {duration:?} seconds.",
                action = payload.action,
                duration = payload.duration,
            ),
        })
    }

    async fn validator(
        &self,
        query: &TestQuery,
        input: Option<&TestPayload>,
    ) -> Result<(), ValidationError> {
        let mut fields = ValidationError::new();

        if let Some(payload) = input {
            if payload.duration < 0.0 {
                fields.insert(
                    "duration".to_owned(),
                    "Duration cannot be negative.".to_owned(),
                );
            }

            if query.name.is_empty() {
                fields.insert("name".to_owned(), "Name cannot be empty.".to_owned());
            }
        } else {
            fields.insert("$body".to_owned(), "A POST Payload is required.".to_owned());
        }

        if fields.is_empty() {
            Ok(())
        } else {
            Err(fields)
        }
    }
}

/// Create a new shop for testing.
pub async fn new_shop() -> Arc<Shop<TestQuery, TestPayload, TestResult, TestMachine>> {
    Shop::new(
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
    .expect("Failed to create the shop.")
}
