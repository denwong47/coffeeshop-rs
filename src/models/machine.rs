use std::future::Future;

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
pub trait Machine<I, O>: Clone + Send + Sized {
    type Future: Future<Output = O> + Send;

    // Required method
    fn call(self, input: I) -> Self::Future;
}
