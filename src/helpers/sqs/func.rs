use crate::{
    helpers,
    models::{message, Ticket},
    CoffeeShopError,
};
use aws_sdk_sqs as sqs;

use super::{encoding, HasSQSConfiguration, StagedReceipt};

const LOG_TARGET: &str = "coffeeshop::helpers::sqs::func";

/// Put a ticket into the AWS SQS queue.
pub async fn put_ticket<Q, I>(
    config: &dyn HasSQSConfiguration,
    input: message::CombinedInput<Q, I>,
    temp_dir: &tempfile::TempDir,
) -> Result<Ticket, CoffeeShopError>
where
    Q: message::QueryType,
    I: serde::de::DeserializeOwned + serde::Serialize,
{
    let client = sqs::Client::new(config.aws_config());

    let serialized_input = helpers::serde::serialize(&input, temp_dir).await?;

    let response = client
        .send_message()
        .queue_url(config.sqs_queue_url())
        .message_body(encoding::encode(&serialized_input.read_to_end().await?).await?)
        .send()
        .await
        .inspect_err(
            |err| crate::error!(target: LOG_TARGET, "Failed to send message: {err}", err = err),
        )
        .map_err(|sdk_err| {
            CoffeeShopError::from_aws_sqs_send_message_error(sdk_err.into_service_error(), config)
        })?;

    crate::info!(
        target: LOG_TARGET,
        "Sent message ID {message_id}.",
        message_id = response.message_id().unwrap_or("(None; this is unexpected?)"),
    );

    response.message_id().map(Ticket::from).ok_or_else(|| {
        CoffeeShopError::UnexpectedAWSResponse(
            "No message ID returned upon sending message.".to_string(),
        )
    })
}

/// Retrieve a ticket from the AWS SQS queue.
pub async fn retrieve_ticket<Q, I>(
    config: &dyn HasSQSConfiguration,
    timeout: Option<tokio::time::Duration>,
) -> Result<StagedReceipt<Q, I>, CoffeeShopError>
where
    Q: message::QueryType,
    I: serde::de::DeserializeOwned + serde::Serialize,
{
    // Call the `receive` method on the `StagedReceipt` struct.
    StagedReceipt::receive(config, timeout).await
}

/// Purge a queue of all messages.
pub async fn purge_tickets(config: &dyn HasSQSConfiguration) -> Result<(), CoffeeShopError> {
    let client = sqs::Client::new(config.aws_config());

    client
        .purge_queue()
        .queue_url(config.sqs_queue_url())
        .send()
        .await
        // TODO Make a more unique error type for this.
        .map_err(|err| CoffeeShopError::AWSSdkError(format!("{:?}", err)))?;

    Ok(())
}

/// Get ticket count.
pub async fn get_ticket_count(config: &dyn HasSQSConfiguration) -> Result<usize, CoffeeShopError> {
    let client = sqs::Client::new(config.aws_config());

    let response = client
        .get_queue_attributes()
        .queue_url(config.sqs_queue_url())
        .attribute_names(sqs::types::QueueAttributeName::ApproximateNumberOfMessages)
        .send()
        .await
        .map_err(|err| CoffeeShopError::AWSSdkError(format!("{:?}", err)))?;

    response
        .attributes
        .ok_or_else(|| CoffeeShopError::UnexpectedAWSResponse("Missing attributes".to_string()))
        .and_then(|attributes| {
            attributes
                .get(&sqs::types::QueueAttributeName::ApproximateNumberOfMessages)
                .ok_or_else(|| {
                    CoffeeShopError::UnexpectedAWSResponse(
                        "Missing approximate number of messages".to_string(),
                    )
                })
                .and_then(|count| {
                    count.parse::<usize>().map_err(|err| {
                        CoffeeShopError::UnexpectedAWSResponse(format!(
                            "Failed to parse the approximate number of messages {count:?}: {err}"
                        ))
                    })
                })
        })
}
