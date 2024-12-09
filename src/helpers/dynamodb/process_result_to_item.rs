//! This module contains implementations on
//! [`dynamodb::operation::put_item::builders::PutItemFluentBuilder`] to convert
//! processing results into DynamoDB items.
//!
//! If the result is a [`Ok<O, _>`], then a `output` field is added to the item
//! with a status code of `200`.
//! If the result is a [`Err<_, CoffeeShopError>`], then an `error` field is added
//! to the item with the status code of the error. The error message is customised
//! by the error type of [`CoffeeShopError::ErrorSchema`].

use super::{ERROR_KEY, OUTPUT_KEY, STATUS_KEY, SUCCESS_KEY, TTL_KEY};
use crate::{
    helpers,
    models::{message::ProcessResult, Ticket},
    CoffeeShopError,
};
use aws_sdk_dynamodb as dynamodb;

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
    async fn report_ticket_success<O: serde::Serialize + Send + Sync + 'static>(
        self,
        partition_key: &str,
        ticket: &Ticket,
        output: O,
        ttl: &tokio::time::Duration,
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
    async fn report_ticket_result<O>(
        self,
        partition_key: &str,
        ticket: &Ticket,
        result: ProcessResult<O>,
        ttl: &tokio::time::Duration,
    ) -> Self::Output
    where
        O: serde::Serialize + Send + Sync + 'static,
    {
        match result {
            Ok(output) => {
                self.report_ticket_success(partition_key, ticket, output, ttl)
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

    async fn report_ticket_success<O: serde::Serialize + Send + Sync + 'static>(
        self,
        partition_key: &str,
        ticket: &Ticket,
        output: O,
        ttl: &tokio::time::Duration,
    ) -> Self::Output {
        let buffer = helpers::serde::serialize(output).await?;

        Ok(add_common_items(self, partition_key, ticket, ttl)
            .item(
                STATUS_KEY,
                dynamodb::types::AttributeValue::N("200".to_owned()),
            )
            .item(SUCCESS_KEY, dynamodb::types::AttributeValue::Bool(true))
            .item(
                OUTPUT_KEY,
                dynamodb::types::AttributeValue::B(dynamodb::primitives::Blob::new(buffer)),
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
