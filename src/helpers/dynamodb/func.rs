//! Helper functions to interact with DynamoDB as a key-value result store.

use aws_sdk_dynamodb as dynamodb;
use itertools::Itertools;
use serde::de::DeserializeOwned;
use std::vec;

use crate::{
    models::{
        message::{ProcessResult, ProcessResultExport},
        Ticket,
    },
    CoffeeShopError,
};

use super::*;

/// Put a processing result into a DynamoDB table.
pub async fn put_item<O>(
    config: &dyn HasDynamoDBConfiguration,
    ticket: &Ticket,
    result: ProcessResult<O>,
    temp_dir: &tempfile::TempDir,
) -> Result<(), CoffeeShopError>
where
    O: serde::Serialize + Send + Sync,
{
    let client = dynamodb::Client::new(config.aws_config());
    let table = config.dynamodb_table();

    client.put_item()
        .table_name(table)
        .report_ticket_result(config.dynamodb_partition_key(), ticket, result, &config.dynamodb_ttl(), temp_dir).await?
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

/// Get items that matches any given partition keys from a DynamoDB table.
///
/// # Safety
///
/// The [BatchGetItem] only supports up to 100 items per request; this function
/// does not check the number of tickets. If the number of tickets exceeds 100,
/// the request will fail.
///
/// [BatchGetItem]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_BatchGetItem.html
pub async fn get_items_by_tickets_unchecked<O, C>(
    config: &C,
    tickets: impl Iterator<Item = &Ticket>,
) -> Result<Vec<(Ticket, ProcessResultExport<O>)>, CoffeeShopError>
where
    O: DeserializeOwned + Send + Sync,
    C: HasDynamoDBConfiguration,
{
    let client = dynamodb::Client::new(config.aws_config());
    let table = config.dynamodb_table();

    let keys = tickets
        .map(|ticket| {
            // This creates a { partition_key: ticket } map for each ticket...
            let mut map = std::collections::HashMap::new();
            map.insert(
                config.dynamodb_partition_key().to_owned(),
                dynamodb::types::AttributeValue::S(ticket.to_string()),
            );
            map
        })
        // ...and collects them into a vector.
        .collect::<Vec<_>>();

    let response =
        // Construct and send the batch get item request.
        client.batch_get_item()
        .request_items(
            // This creates a mapper that requests from this table...
            config.dynamodb_table(),
            // ...all items with the given keys from the above vector.
            dynamodb::types::KeysAndAttributes::builder().set_keys(Some(keys)).build().map_err(
                |err| {
                    crate::error!(
                        "Failed to build the keys and attributes for the batch get item request for table {}. Error: {:?}",
                        table,
                        err
                    );

                    // TODO - Implement a more specific error type for DynamoDB errors.
                    CoffeeShopError::AWSSdkError(format!("{:?}", err))
                }
            )?
        )
        .send().await
        .map_err(
            // TODO - Implement a more specific error type for DynamoDB errors.
            |sdk_err| CoffeeShopError::AWSSdkError(format!("{:?}", sdk_err))
        )?;

    let consumed_capacity = response
        .consumed_capacity()
        .iter()
        .fold(0., |acc, capacity| {
            acc + capacity.capacity_units().unwrap_or_default()
        });

    // Iterate the responses and collect the results.
    if let Some(mut table_mapper) = response.responses {
        if let Some(results) = table_mapper.remove(table) {
            crate::info!(
                "Retrieved {} processing results from the DynamoDB table {}. Consumed {:?} capacity units.",
                results.len(),
                table,
                consumed_capacity
            );

            // Iterate the results and convert them into a vector of (ticket, result) tuples.
            return results
                .into_iter()
                .map(|map|
                        // Convert all the items from the table into a (ticket, result) tuple.
                        map.to_process_result(config.dynamodb_partition_key()))
                .collect::<Result<Vec<_>, _>>();
        }
    }

    crate::warn!(
        "No processing results found for the given tickets in the DynamoDB table {}. Consumed {:?} capacity units.",
        table,
        consumed_capacity,
    );

    Ok(vec![])
}

/// Get items that matches any given partition keys from a DynamoDB table.
pub async fn get_items_by_tickets<O, C>(
    config: &C,
    tickets: impl ExactSizeIterator<Item = &Ticket>,
) -> Result<Vec<(Ticket, ProcessResultExport<O>)>, CoffeeShopError>
where
    O: DeserializeOwned + Send + Sync,
    C: HasDynamoDBConfiguration,
{
    if tickets.len() == 0 {
        // If there are no tickets, return an empty vector.
        // This is mandatory because `itertools.chunks` panics on empty iterators.
        return Ok(vec![]);
    }

    let chunks_count = (tickets.len() as f32 / 100.).ceil() as usize;
    let chunk_size = (tickets.len() as f32 / chunks_count as f32).ceil() as usize;
    let chunks = tickets.chunks(chunk_size);

    futures::future::try_join_all(
        // Iterate the chunks and get the items for each chunk.
        chunks
            .into_iter()
            .map(|chunk| get_items_by_tickets_unchecked::<O, _>(config, chunk)),
    )
    .await
    .map(|results| results.into_iter().flatten().collect())
}
