//! Test models for testing the message module.

use crate::{
    models::{message, Machine, Ticket},
    CoffeeMachineError,
};
use axum::http;
use serde::{Deserialize, Serialize};

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
        if let Some(payload) = input {
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
            } else if query.name.is_empty() {
                return Err(CoffeeMachineError::new(
                    http::StatusCode::UNPROCESSABLE_ENTITY,
                    "MissingName".to_owned(),
                    Some(serde_json::json!({
                        "message": "No name was provided.",
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
        } else {
            Err(CoffeeMachineError::new(
                http::StatusCode::BAD_REQUEST,
                "MissingInput".to_owned(),
                Some(serde_json::json!({
                    "message": "No payload was provided.",
                })),
            ))
        }
    }
}
