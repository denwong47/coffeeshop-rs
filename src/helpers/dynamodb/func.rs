//! Helper functions to interact with DynamoDB as a key-value result store.

use aws_sdk_dynamodb as dynamodb;
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
pub async fn put_process_result<O>(
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
pub async fn get_items_by_tickets_unchecked<C>(
    config: &C,
    tickets: impl Iterator<Item = &Ticket>,
    projection_expression: Option<&[String]>,
) -> Result<Vec<DynamoDBItem>, CoffeeShopError>
where
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
            dynamodb::types::KeysAndAttributes::builder()
            .set_keys(Some(keys))
            .set_attributes_to_get(projection_expression.map(|attrs| attrs.to_vec()))
            .build()
            .map_err(
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
            return Ok(results);
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
pub async fn get_items_by_tickets<C>(
    config: &C,
    tickets: impl ExactSizeIterator<Item = &Ticket>,
    projection_expression: Option<&[String]>,
) -> Result<Vec<DynamoDBItem>, CoffeeShopError>
where
    C: HasDynamoDBConfiguration,
{
    if tickets.len() == 0 {
        // If there are no tickets, return an empty vector.
        // This is mandatory because `itertools.chunks` panics on empty iterators.
        return Ok(vec![]);
    }

    let chunks_count = (tickets.len() as f32 / 100.).ceil() as usize;
    let chunk_size = (tickets.len() as f32 / chunks_count as f32).ceil() as usize;
    // We can't use itertools here because `chunks` actually uses RefCell, which is not Send.
    let chunks = {
        let mut tickets = tickets.collect::<Vec<_>>();
        let mut chunks = vec![];

        loop {
            if tickets.len() <= chunk_size {
                let mut middleman = vec![];
                std::mem::swap(&mut tickets, &mut middleman);
                chunks.push(middleman);
                break;
            } else {
                chunks.push(tickets.split_off(chunk_size));
            }
        }

        chunks
    };

    futures::future::try_join_all(
        // Iterate the chunks and get the items for each chunk.
        chunks.into_iter().map(|chunk| {
            get_items_by_tickets_unchecked::<_>(config, chunk.into_iter(), projection_expression)
        }),
    )
    .await
    .map(|results| results.into_iter().flatten().collect())
}

/// Get the processing results that matches any given partition keys from a DynamoDB table.
pub async fn get_process_results_by_tickets<O, C>(
    config: &C,
    tickets: impl ExactSizeIterator<Item = &Ticket>,
) -> Result<Vec<(Ticket, ProcessResultExport<O>)>, CoffeeShopError>
where
    O: DeserializeOwned + Send + Sync,
    C: HasDynamoDBConfiguration,
{
    get_items_by_tickets(config, tickets, None)
        .await
        .and_then(|items| {
            items
                .into_iter()
                .map(|item| item.to_process_result(config.dynamodb_partition_key()))
                .collect::<Result<Vec<_>, _>>()
        })
}
/// Get the statuses that matches any given partition keys from a DynamoDB table.
pub async fn get_process_successes_by_tickets<C>(
    config: &C,
    tickets: impl ExactSizeIterator<Item = &Ticket>,
) -> Result<Vec<(Ticket, bool)>, CoffeeShopError>
where
    C: HasDynamoDBConfiguration,
{
    let projection_expression = vec![
        config.dynamodb_partition_key().to_owned(),
        SUCCESS_KEY.to_owned(),
    ];

    get_items_by_tickets(config, tickets, Some(&projection_expression))
        .await
        .and_then(|items| {
            items
                .into_iter()
                .map(|item| item.to_process_status(config.dynamodb_partition_key()))
                .collect::<Result<Vec<_>, _>>()
        })
}

/// Get a single processing result that matches the given partition key from a DynamoDB table.
/// This function currently is a convenience wrapper around [`get_process_results_by_tickets`];
/// which could take a bit more computation time than necessary, but reduces the maintenance
/// overhead of having to maintain two separate functions.
///
/// # Note
///
/// This function is not optimized for performance; it is recommended to use
/// [`get_process_results_by_tickets`] if you need to get multiple results.
pub async fn get_process_result_by_ticket<O, C>(
    config: &C,
    ticket: &Ticket,
) -> Result<ProcessResultExport<O>, CoffeeShopError>
where
    O: DeserializeOwned + Send + Sync,
    C: HasDynamoDBConfiguration,
{
    get_process_results_by_tickets(config, std::iter::once(ticket))
        .await
        .and_then(|mut results| {
            results
                .pop()
                .and_then(
                    |(found_ticket, result)| {
                        if found_ticket == *ticket {
                            Some(result)
                        } else {
                            crate::error!(
                                "The ticket {} does not match the found ticket {} in the DynamoDB table {}. Reporting as not found; this should not happen.",
                                ticket,
                                found_ticket,
                                config.dynamodb_table()
                            );
                            None
                        }
                    }
                )
                .ok_or_else(|| {
                    crate::warn!(
                        "No processing result found for the given ticket {} in the DynamoDB table {}.",
                        ticket,
                        config.dynamodb_table()
                    );

                    CoffeeShopError::ResultNotFound(ticket.to_string())
                })
        })
}
