use std::future::Future;

use super::message;

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
pub trait Machine<Q, I, O>: Clone + Send + Sized
where
    Q: message::QueryType,
    I: serde::de::DeserializeOwned + serde::Serialize,
    O: serde::Serialize + serde::de::DeserializeOwned,
{
    /// Required method for the [`Machine`] trait.
    ///
    /// A [`Machine`] is expected to process the input and return the output; if an error
    /// occurs, it should return a [`CoffeeMachineError`].
    fn call(&self, query: &Q, input: Option<&I>)
        -> impl Future<Output = message::MachineResult<O>>;
}
