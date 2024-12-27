use crate::{helpers, models::Ticket, CoffeeShopError};

use aws_sdk_dynamodb as dynamodb;

/// The key for the status of the processing result.
const SUCCESS_KEY: &str = "success";

/// The key for the status code of the processing result.
const STATUS_KEY: &str = "status_code";

/// The key for the output of the processing result.
const OUTPUT_KEY: &str = "output";

/// The key for the error of the processing result.
const ERROR_KEY: &str = "error";

/// The key for the time-to-live of the processing result.
const TTL_KEY: &str = "ttl";

/// Add items common to both
/// [`report_ticket_success`](ToItem::report_ticket_success) and
/// [`report_ticket_failure`](ToItem::report_ticket_failure) to the fluent
/// builder.
fn add_common_items(
    builder: dynamodb::operation::put_item::builders::PutItemFluentBuilder,
    partition_key: &str,
    ticket: &Ticket,
    ttl: &tokio::time::Duration,
) -> dynamodb::operation::put_item::builders::PutItemFluentBuilder {
    // Calculate the expiry time.
    // If the final time exceeds the maximum value, use the maximum value instead.
    let expiry = chrono::Duration::from_std(*ttl).map_or_else(
        |_| chrono::DateTime::<chrono::Utc>::MAX_UTC,
        |duration| chrono::Utc::now() + duration,
    );
    builder
        .item(
            partition_key,
            dynamodb::types::AttributeValue::S(ticket.to_owned()),
        )
        .item(
            TTL_KEY,
            dynamodb::types::AttributeValue::N(expiry.timestamp().to_string()),
        )
}

/// Convert a processing result into a DynamoDB item.
#[async_trait::async_trait]
pub trait ToItem: Sized {
    type Output;

    /// Convert the successful processing result into a DynamoDB item.
    async fn report_ticket_success<O: serde::Serialize + Send + Sync>(
        self,
        partition_key: &str,
        ticket: &Ticket,
        output: O,
        ttl: &tokio::time::Duration,
        temp_dir: &tempfile::TempDir,
    ) -> Self::Output;

    /// Convert the failed processing result into a DynamoDB item.
    async fn report_ticket_failure(
        self,
        partition_key: &str,
        ticket: &Ticket,
        error: CoffeeShopError,
        ttl: &tokio::time::Duration,
    ) -> Self::Output;

    /// Convert the processing result into a DynamoDB item.
    async fn report_ticket_result(
        self,
        partition_key: &str,
        ticket: &Ticket,
        result: Result<impl serde::Serialize + Send + Sync, CoffeeShopError>,
        ttl: &tokio::time::Duration,
        temp_dir: &tempfile::TempDir,
    ) -> Self::Output {
        match result {
            Ok(output) => {
                self.report_ticket_success(partition_key, ticket, output, ttl, temp_dir)
                    .await
            }
            Err(error) => {
                self.report_ticket_failure(partition_key, ticket, error, ttl)
                    .await
            }
        }
    }
}

#[async_trait::async_trait]
impl ToItem for dynamodb::operation::put_item::builders::PutItemFluentBuilder {
    type Output = Result<Self, CoffeeShopError>;

    async fn report_ticket_success<O: serde::Serialize + Send + Sync>(
        self,
        partition_key: &str,
        ticket: &Ticket,
        output: O,
        ttl: &tokio::time::Duration,
        temp_dir: &tempfile::TempDir,
    ) -> Self::Output {
        let buffer = helpers::serde::serialize(&output, temp_dir).await?;

        Ok(add_common_items(self, partition_key, ticket, ttl)
            .item(
                STATUS_KEY,
                dynamodb::types::AttributeValue::N("200".to_owned()),
            )
            .item(SUCCESS_KEY, dynamodb::types::AttributeValue::Bool(true))
            .item(
                OUTPUT_KEY,
                dynamodb::types::AttributeValue::B(dynamodb::primitives::Blob::new(
                    buffer.read_to_end().await?,
                )),
            ))
    }

    async fn report_ticket_failure(
        self,
        partition_key: &str,
        ticket: &Ticket,
        error: CoffeeShopError,
        ttl: &tokio::time::Duration,
    ) -> Self::Output {
        let error_body =
            serde_json::to_string(&error.as_json())
            // Potentially unsafe;
            // however there is very little we can do if the error cannot be serialized.
            .expect("Failed to serialize the error from the processing result. Please check that the error type is serializable.")
        ;

        Ok(add_common_items(self, partition_key, ticket, ttl)
            .item(
                STATUS_KEY,
                dynamodb::types::AttributeValue::N(error.status_code().as_u16().to_string()),
            )
            .item(SUCCESS_KEY, dynamodb::types::AttributeValue::Bool(false))
            .item(ERROR_KEY, dynamodb::types::AttributeValue::S(error_body)))
    }
}
