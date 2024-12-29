//! Helper functions for the [`Shop`](crate::models::Shop) to put processed results into
//! DynamoDB.
//!

/// The key for the status of the processing result.
const SUCCESS_KEY: &str = "success";

/// The key for the status code of the processing result.
const STATUS_KEY: &str = "status_code";

/// The key for the output of the processing result.
const OUTPUT_KEY: &str = "output";

/// The key for the error of the processing result.
const ERROR_KEY: &str = "error";

/// The key for the time-to-live of the processing result.
const TTL_KEY: &str = "ttl";

mod config;
pub use config::*;

mod process_result_to_item;
pub use process_result_to_item::*;

mod item_to_process_result;
pub use item_to_process_result::*;

mod func;
pub use func::*;

#[cfg(test)]
mod tests;
