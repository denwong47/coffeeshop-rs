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

/// Re-export the helpers for the test models.
#[allow(unused_imports)]
pub use crate::helpers::{
    aws::HasAWSSdkConfig, dynamodb::HasDynamoDBConfiguration, sqs::HasSQSConfiguration,
};

use super::message::QueryType;

const LOG_TARGET: &str = "coffeeshop::models::test";

/// The shop type for testing.
pub type TestShop = Shop<TestQuery, TestPayload, TestResult, TestMachine>;

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

#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TestQuery {
    pub name: String,
    #[serde_as(as = "Option<serde_with::DurationSecondsWithFrac<f64>>")]
    pub timeout: Option<tokio::time::Duration>,
    #[serde(rename = "async")]
    #[serde(default)]
    pub is_async: bool,
}

impl message::QueryType for TestQuery {
    fn get_timeout(&self) -> Option<tokio::time::Duration> {
        self.timeout
    }

    fn is_async(&self) -> bool {
        dbg!(self);
        self.is_async
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

impl Default for TestMachine {
    fn default() -> Self {
        Self::new()
    }
}

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
pub async fn new_shop(barista_count: usize) -> Arc<TestShop> {
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
        barista_count,
    )
    .await
    .expect("Failed to create the shop.")
}

/// Send a HTTP request to the shop.
#[allow(dead_code)]
pub async fn send_request<Q: QueryType + Clone>(
    shop: &Arc<TestShop>,
    method: http::Method,
    path: &str,
    query: Option<Q>,
    body: Option<(&str, &str)>,
) -> reqwest::Result<reqwest::Response> {
    const ATTEMPT_COUNT: usize = 3;

    fn request_builder<Q: QueryType>(
        shop: &Arc<TestShop>,
        method: http::Method,
        path: &str,
        query: Option<Q>,
        body: Option<(&str, &str)>,
    ) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        let mut builder = client.request(
            method.clone(),
            format!(
                "http://localhost:{port}{path}",
                port = shop.config.port,
                path = path,
            ),
        );

        builder = if let Some(query) = query {
            builder.query(&query)
        } else {
            builder
        };

        if let Some((mimetype, payload)) = body {
            builder
                .body(payload.to_owned())
                .header(http::header::CONTENT_TYPE, mimetype)
        } else {
            builder
        }
    }

    for attempt in 0..ATTEMPT_COUNT {
        crate::info!(
            "Sending {method:?} request to {path}...",
            method = method,
            path = path,
        );
        let result = request_builder(shop, method.clone(), path, query.clone(), body)
            .send()
            .await;

        if let Ok(response) = &result {
            if response.status() == http::StatusCode::REQUEST_TIMEOUT {
                crate::warn!(
                    "Request to {path} timed out. Retrying... ({attempt}/{total})",
                    path = path,
                    attempt = attempt + 1,
                    total = ATTEMPT_COUNT,
                );
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                continue;
            }

            crate::info!(
                "Got {status:?} response from {method:?} {path} containing {size} bytes.",
                status = response.status(),
                method = method,
                path = path,
                size = response.content_length().unwrap_or(0),
            );

            return result;
        } else {
            crate::warn!(
                "Failed to send {method:?} request to {path}. Retrying... ({attempt}/{total})",
                method = method,
                path = path,
                attempt = attempt + 1,
                total = ATTEMPT_COUNT,
            )
        }
    }

    panic!(
        "Failed to send {method:?} request to {path}.",
        method = method,
        path = path,
    )
}

/// Send a HTTP request with a JSON body to the shop.
#[allow(dead_code)]
pub async fn send_json_request<Q: QueryType + Clone, S: Serialize>(
    shop: &Arc<TestShop>,
    method: http::Method,
    path: &str,
    query: Option<Q>,
    obj: S,
) -> reqwest::Result<reqwest::Response> {
    assert_ne!(
        method,
        http::Method::GET,
        "GET requests cannot have a body."
    );

    let payload = serde_json::to_string(&obj).expect("Failed to serialize the object.");
    let mimetype = "application/json";

    send_request(shop, method, path, query, Some((mimetype, &payload))).await
}
