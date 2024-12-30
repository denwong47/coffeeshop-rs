use crate::{CoffeeMachineError, ValidationError};
use axum::http;

use serde::{de::DeserializeOwned, Serialize};

use super::message;

#[cfg(doc)]
use super::Waiter;

/// A trait that defines the behavior of a coffee machine, i.e. the function
/// that will be called when a ticket is received, and outputs the result
/// to the DynamoDB table.
///
/// # Note
///
/// Async closures are not expected to be in Stable until at least Q1 2025,
/// therefore we could not globally implement the `AsyncFn` trait yet for this
/// trait.
///
/// To use this trait, you must implement the `Machine` trait for your struct
/// and define the `call` method, which can be a simple wrapper around an async
/// function.
#[async_trait::async_trait]
pub trait Machine<Q, I, O>: Send + Sync + Sized
where
    Q: message::QueryType,
    I: DeserializeOwned + Serialize + Send + Sync,
    O: DeserializeOwned + Serialize + Send + Sync,
{
    /// Required method for the [`Machine`] trait.
    ///
    /// A [`Machine`] is expected to process the input and return the output; if an error
    /// occurs, it should return a [`CoffeeMachineError`].
    async fn call(&self, query: &Q, input: Option<&I>) -> message::MachineResult<O>;

    /// Validate the input before processing.
    ///
    /// This prevents erroronous input from being sent to the SQS in the first place;
    /// and the [`Waiter`] will return a [`http::StatusCode::UNPROCESSABLE_ENTITY`] response
    /// with the given [`ValidationError`] as [details](serde_json::Value).
    async fn validator(&self, query: &Q, input: Option<&I>) -> Result<(), ValidationError>;

    /// The default validator implementation that wraps the [details](serde_json::Value)
    /// in a [`CoffeeMachineError`] and returns it.
    ///
    /// Due to the signature of this method, you can optionally call it at the top of your
    /// [`call`](Self::call) method to ensure that the input is valid before processing.
    /// This is useful if you have other queue inputs that are not validated by
    /// the [`Waiter`].
    ///
    /// # Note
    ///
    /// You can customise this method to return a different error as needed, as long
    /// as it returns a [`CoffeeMachineError`].
    async fn validate(&self, query: &Q, input: Option<&I>) -> Result<(), CoffeeMachineError> {
        self.validator(query, input)
            .await
            .map_err(|details| CoffeeMachineError::new(
                http::StatusCode::UNPROCESSABLE_ENTITY,
                "ValidationError".to_owned(),
                Some(serde_json::json!(
                    {
                        "message": format!(
                            "Input cannot be validated. Please see details for {count} invalid field{s} below.",
                            count = details.len(),
                            s = if details.len() == 1 { "" } else { "s" },
                        ),
                        "fields": serde_json::to_value(details).expect(
                            "Failed to serialize the validation error details; this should not be possible.",
                        )
                    }
                )),
            ))
    }
}
