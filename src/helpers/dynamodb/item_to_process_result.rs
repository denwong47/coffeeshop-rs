use crate::{
    helpers,
    models::{message::ProcessResult, Ticket},
    CoffeeMachineError, CoffeeShopError,
};
use axum::http;
use serde::de::DeserializeOwned;

use super::{ERROR_KEY, OUTPUT_KEY, STATUS_KEY, SUCCESS_KEY};

use aws_sdk_dynamodb::types::AttributeValue;

/// Trait for converting an item to a process result.
pub trait ToProcessResult<O>
where
    O: DeserializeOwned + Send + Sync,
{
    /// Attempt to convert the item into a process result.
    ///
    /// The return type of this has a nested [`Result`]:
    /// - The outer [`Result<(Ticket, _)`] is the result of the conversion.
    /// - The inner [`ProcessResult<O>`] is the actual processing result.
    fn to_process_result(
        self,
        partition_key: &str,
    ) -> Result<(Ticket, ProcessResult<O>), CoffeeShopError>;
}

impl<O> ToProcessResult<O> for std::collections::HashMap<String, AttributeValue>
where
    O: DeserializeOwned + Send + Sync,
{
    fn to_process_result(
        mut self,
        partition_key: &str,
    ) -> Result<(Ticket, ProcessResult<O>), CoffeeShopError> {
        match (
            self.remove(partition_key),
            self.remove(SUCCESS_KEY),
            self.remove(STATUS_KEY),
            self.remove(OUTPUT_KEY),
            self.remove(ERROR_KEY),
        ) {
            // Successful processing result.
            (
                Some(AttributeValue::S(ticket)),
                Some(AttributeValue::Bool(true)),
                Some(AttributeValue::N(status)),
                Some(AttributeValue::B(blob)),
                None,
            ) => {
                let output = helpers::serde::deserialize::<O>(blob.into_inner())?;

                crate::info!(
                    "Successfully retrieved processing result for ticket {}. Status: {}.",
                    ticket,
                    status,
                );

                Ok((ticket, Ok(output)))
            }
            // Failed processing result.
            (
                Some(AttributeValue::S(ticket)),
                Some(AttributeValue::Bool(false)),
                Some(AttributeValue::N(status)),
                None,
                Some(AttributeValue::S(error_json)),
            ) => {
                let error: CoffeeMachineError = serde_json::from_str(&error_json).inspect_err(
                    |_| crate::error!(
                        "Encountered an unparsable error schema for ticket {}. Status: {}. Error: {:?}",
                        ticket,
                        status,
                        error_json,
                    )
                ).unwrap_or_else(
                    |_|
                        CoffeeMachineError::new(
                            http::StatusCode::INTERNAL_SERVER_ERROR,
                            "UnknownProcessingError".to_owned(),
                            Some(serde_json::json!({
                                "message": "A processing error had occurred, but the error message cannot be parsed; could not report the actual error.",
                                "original": error_json,
                            }))
                        )
                );

                crate::warn!(
                    "Successfully retrieved error schema for ticket {}. Status: {}. Error: {:?}",
                    ticket,
                    status,
                    error,
                );

                Ok((ticket, Err(CoffeeShopError::ErrorSchema(error))))
            }
            _ => Err(CoffeeShopError::DynamoDBMalformedItem(
                "A map was retrieved, but its structure could not be parsed.".to_string(),
            )),
        }
    }
}
