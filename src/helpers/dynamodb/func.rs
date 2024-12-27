//! Helper functions to interact with DynamoDB as a key-value result store.

use crate::{helpers::aws, models::Ticket, CoffeeShopError};
use aws_sdk_dynamodb as dynamodb;

use super::ToItem;

/// Put a processing result into a DynamoDB table.
pub async fn put_item<O>(
    table: &str,
    partition_key: &str,
    config: &aws::SdkConfig,
    ticket: &Ticket,
    result: Result<O, CoffeeShopError>,
    ttl: &tokio::time::Duration,
    temp_dir: &tempfile::TempDir,
) -> Result<(), CoffeeShopError>
where
    O: serde::Serialize + Send + Sync,
{
    let client = dynamodb::Client::new(config);

    client.put_item()
        .table_name(table)
        .report_ticket_result(partition_key, ticket, result, ttl, temp_dir).await?
        .send()
        .await
        .map_err(|sdk_err| {
            crate::error!(
                "Failed to put the processing result for ticket {} into the DynamoDB table {}. Error: {:?}",
                ticket,
                table,
                sdk_err
            );

            // TODO - Implement a more specific error type for DynamoDB errors.
            CoffeeShopError::AWSSdkError(format!("{:?}", sdk_err))
        })
        .map(
            |response| {
                crate::info!(
                    "Successfully put the processing result for ticket {} into the DynamoDB table {}. Consumed {:?} capacity units.",
                    ticket,
                    table,
                    response.consumed_capacity().map(|capacity| capacity.capacity_units()).unwrap_or_default()
                )
            }
        )
}
