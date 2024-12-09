use std::future::Future;

/// Retry the execution of an async function until it returns a value or an error.
pub async fn until<T, E, FutT>(
    operation_name: &str,
    task_factory: impl Fn() -> FutT,
    max_retries: usize,
    predicate: impl Fn(&Result<T, E>) -> bool,
) -> Result<T, E>
where
    T: std::fmt::Debug,
    E: std::fmt::Debug,
    FutT: Future<Output = Result<T, E>>,
{
    let mut attempt = 0;
    loop {
        let result = task_factory().await;

        if predicate(&result) {
            return result;
        }

        crate::info!(
            "Attempt {}/{} to {operation_name} failed: {:?}",
            attempt + 1,
            max_retries,
            &result,
        );

        attempt += 1;

        if attempt >= max_retries {
            return result;
        }
    }
}

/// Retry the execution of an async function until it returns a [`Ok`] value.
///
/// # Parameters
///
/// - `operation_name` - The name of the operation to be performed. This will be used in the logs.
///                      It will be presented in the format of `Attempt to {operation_name} failed`.
/// - `task_factory` - A function that returns a future that will be executed.
/// - `max_retries` - The maximum number of retries before giving up.
pub async fn until_ok<T, E, FutT>(
    operation_name: &str,
    task_factory: impl Fn() -> FutT,
    max_retries: usize,
) -> Result<T, E>
where
    T: std::fmt::Debug,
    E: std::fmt::Debug,
    FutT: Future<Output = Result<T, E>>,
{
    until(operation_name, task_factory, max_retries, |result| {
        result.is_ok()
    })
    .await
}
