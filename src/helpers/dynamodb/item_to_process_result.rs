use crate::{
    errors::ErrorSchema,
    helpers,
    models::{message::ProcessResultExport, Ticket},
    CoffeeShopError,
};
use axum::http;
use serde::de::DeserializeOwned;

use super::{ERROR_KEY, OUTPUT_KEY, STATUS_KEY, SUCCESS_KEY};

use aws_sdk_dynamodb::types::AttributeValue;

#[cfg(doc)]
use crate::models::message::ProcessResult;

/// Trait for converting an item to a process result.
pub trait ToProcessResult {
    /// Attempt to check the status of the item.
    ///
    /// This function do not require the full result to be present in the item;
    /// only the ticket and success status are needed. Only tickets with a success
    /// status will be returned, regardless of whether it is `true` or `false`.
    ///
    /// This does not consume the item; a minimal cloning is performed on the ticket.
    fn to_process_status(&self, partition_key: &str) -> Result<(Ticket, bool), CoffeeShopError>;

    /// Attempt to convert the item into a process result.
    ///
    /// The return type of this has a nested [`Result`]:
    /// - The outer [`Result<(Ticket, _)`] is the result of the conversion.
    /// - The inner [`ProcessResultExport<O>`] is the actual processing result.
    ///
    /// Only the inner [`ErrorSchema`] is preserved, distinguishing it from a local error.
    /// Not being wrapped in a [`CoffeeShopError::ErrorSchema`], this also
    /// ensure that the error can be [`Clone`]d and serialized into a
    /// [`Response`](axum::http::Response), since the original error could
    /// contain non-serializable types or non-static lifetimes.
    fn to_process_result<O>(
        self,
        partition_key: &str,
    ) -> Result<(Ticket, ProcessResultExport<O>), CoffeeShopError>
    where
        O: DeserializeOwned + Send + Sync + 'static;
}

impl ToProcessResult for std::collections::HashMap<String, AttributeValue> {
    fn to_process_status(&self, partition_key: &str) -> Result<(Ticket, bool), CoffeeShopError> {
        match (self.get(partition_key), self.get(SUCCESS_KEY)) {
            (Some(AttributeValue::S(ticket)), Some(AttributeValue::Bool(success))) => {
                Ok((ticket.clone(), *success))
            }
            _ => Err(CoffeeShopError::AWSDynamoDBMalformedItem(
                "A map was retrieved, but its structure could not be parsed.".to_string(),
            )),
        }
    }

    fn to_process_result<O>(
        mut self,
        partition_key: &str,
    ) -> Result<(Ticket, ProcessResultExport<O>), CoffeeShopError>
    where
        O: DeserializeOwned + Send + Sync + 'static,
    {
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
                let error: ErrorSchema = serde_json::from_str(&error_json).inspect_err(
                    |_| crate::error!(
                        "Encountered an unparsable error schema for ticket {}. Status: {}. Error: {:?}",
                        ticket,
                        status,
                        error_json,
                    )
                ).unwrap_or_else(
                    |_|
                        ErrorSchema::new(
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

                Ok((ticket, Err(error)))
            }
            _ => Err(CoffeeShopError::AWSDynamoDBMalformedItem(
                "A map was retrieved, but its structure could not be parsed.".to_string(),
            )),
        }
    }
}
