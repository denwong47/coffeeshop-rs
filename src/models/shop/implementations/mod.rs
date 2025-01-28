//! This module contains the implementations of the shop models.
//!
//! Since implementations do not need to be referenced, these modules do not need to be
//! public.

use super::Shop;

mod has_aws_sdk_config;
mod has_dynamodb_config;
mod has_sqs_config;

mod collection_point;
pub use collection_point::CollectionPoint;
