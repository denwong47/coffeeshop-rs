//! Helper functions for the [`Shop`](crate::models::Shop) to put processed results into
//! DynamoDB.
//!

mod process_result_to_item;
pub use process_result_to_item::*;

mod func;
pub use func::*;

#[cfg(test)]
mod tests;
