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
) -> Result<(), CoffeeShopError>
where
    O: serde::Serialize + Send + Sync + 'static,
{
    let client = dynamodb::Client::new(config.aws_config());
    let table = config.dynamodb_table();

    client.put_item()
        .table_name(table)
        .report_ticket_result(config.dynamodb_partition_key(), ticket, result, &config.dynamodb_ttl()).await?
        .send()
        .await
        .map_err(|sdk_err| {
            crate::error!(
                "Failed to put the processing result for ticket {} into the DynamoDB table {}. Error: {:?}",
                ticket,
                table,
                sdk_err
            );

            CoffeeShopError::from_aws_dynamodb_error(sdk_err.into_service_error().into(), config)
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

                    CoffeeShopError::from_aws_dynamodb_error(err.into(), config)
                }
            )?
        )
        .send().await
        .map_err(
            |sdk_err| CoffeeShopError::from_aws_dynamodb_error(sdk_err.into_service_error().into(), config)
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

/// Split an iterator of tickets into chunks of 100 tickets each.
pub fn tickets_into_chunks<'t>(
    tickets: impl ExactSizeIterator<Item = &'t Ticket>,
) -> Vec<Vec<&'t Ticket>> {
    let chunks_count = (tickets.len() as f32 / 100.).ceil() as usize;
    let chunk_size = (tickets.len() as f32 / chunks_count as f32).floor() as usize;
    let mut remainder = tickets.len() % chunk_size;
    // We can't use itertools here because `chunks` actually uses RefCell, which is not Send.

    let mut tickets = tickets.collect::<Vec<_>>();
    let mut chunks = vec![];

    loop {
        let target_size = if remainder > 0 {
            remainder -= 1;
            chunk_size + 1
        } else {
            chunk_size
        };

        if tickets.len() <= target_size {
            // This allows us to avoid cloning the tickets without changing
            // the ownership of the original vector.
            let mut middleman = vec![];
            std::mem::swap(&mut tickets, &mut middleman);
            chunks.push(middleman);
            break;
        } else {
            chunks.push(tickets.split_off(tickets.len() - target_size));
        }
    }

    chunks
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

    let chunks = tickets_into_chunks(tickets);

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
    O: DeserializeOwned + Send + Sync + 'static,
    C: HasDynamoDBConfiguration,
{
    let items = get_items_by_tickets(config, tickets, None).await?;

    let dynamodb_partition_key = config.dynamodb_partition_key().to_owned();
    let handle = tokio::task::spawn_blocking(move || {
        items
            .into_iter()
            .map(|item| item.to_process_result(&dynamodb_partition_key))
            .collect::<Result<Vec<_>, _>>()
    });

    handle.await.unwrap_or_else(|err| {
        crate::error!(
            "The thread to convert the DynamoDB items into processing results panicked. Error: {:?}",
            err
        );

        Err(CoffeeShopError::ThreadResourceError(err.to_string()))
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
    O: DeserializeOwned + Send + Sync + 'static,
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a vector of [`Ticket`]s for testing.
    fn build_tickets(size: usize) -> Vec<Ticket> {
        (1..=size).map(|i| i.to_string()).collect()
    }

    macro_rules! create_test {
        (
            $name:ident(
                $size:literal,
                $expected:expr
            )
        ) => {
            #[test]
            fn $name() {
                let tickets = build_tickets($size);

                let chunks = tickets_into_chunks(tickets.iter());
                let chunk_sizes = chunks.iter().map(|chunk| chunk.len()).collect::<Vec<_>>();

                assert_eq!(&chunk_sizes, $expected);
            }
        };
    }

    create_test!(test_1_tickets(1, &[1]));
    create_test!(test_2_tickets(2, &[2]));
    create_test!(test_99_tickets(99, &[99]));
    create_test!(test_100_tickets(100, &[100]));
    create_test!(test_101_tickets(101, &[51, 50]));
    create_test!(test_102_tickets(102, &[51, 51]));
    create_test!(test_103_tickets(103, &[52, 51]));
    create_test!(test_199_tickets(199, &[100, 99]));
    create_test!(test_200_tickets(200, &[100, 100]));
    create_test!(test_201_tickets(201, &[67, 67, 67]));
    create_test!(test_202_tickets(202, &[68, 67, 67]));
    create_test!(test_1234_tickets(
        1234,
        &[95, 95, 95, 95, 95, 95, 95, 95, 95, 95, 95, 95, 94]
    ));
}
