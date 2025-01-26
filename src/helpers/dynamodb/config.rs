use crate::helpers::aws::HasAWSSdkConfig;
use std::sync::Arc;

use crate::helpers::aws;

#[cfg(doc)]
use crate::models::Shop;

/// A [`HasDynamoDBConfiguration`] contains the configuration for the DynamoDB table
/// that the [`Shop`] will be using.
pub trait HasDynamoDBConfiguration: HasAWSSdkConfig {
    /// The name of the DynamoDB table.
    fn dynamodb_table(&self) -> &str;

    /// The partition key of the DynamoDB table.
    fn dynamodb_partition_key(&self) -> &str;

    /// The time-to-live (TTL) duration for the items in the DynamoDB table.
    fn dynamodb_ttl(&self) -> tokio::time::Duration;

    /// Extract the configuration as a separate struct.
    ///
    /// This is useful if the main configuration struct is too large, or it
    /// lacks certain traits such as [`Send`] or [`Sync`].
    fn dynamodb_configuration(&self) -> DynamoDBConfiguration {
        DynamoDBConfiguration {
            table: self.dynamodb_table().to_owned(),
            partition_key: self.dynamodb_partition_key().to_owned(),
            ttl: self.dynamodb_ttl(),
            aws_config: self.aws_config().clone(),
        }
    }
}

/// Passthrough the [`HasDynamoDBConfiguration`] trait for [`Arc`] pointers.
impl<T> HasDynamoDBConfiguration for Arc<T>
where
    T: HasDynamoDBConfiguration,
{
    fn dynamodb_table(&self) -> &str {
        (**self).dynamodb_table()
    }

    fn dynamodb_partition_key(&self) -> &str {
        (**self).dynamodb_partition_key()
    }

    fn dynamodb_ttl(&self) -> tokio::time::Duration {
        (**self).dynamodb_ttl()
    }

    fn dynamodb_configuration(&self) -> DynamoDBConfiguration {
        (**self).dynamodb_configuration()
    }
}

/// A minimal implementation of [`HasDynamoDBConfiguration`] for testing purposes, or
/// to use this module without a full [`Shop`] configuration.
pub struct DynamoDBConfiguration {
    pub table: String,
    pub partition_key: String,
    pub ttl: tokio::time::Duration,
    pub aws_config: aws::SdkConfig,
}

impl HasAWSSdkConfig for DynamoDBConfiguration {
    fn aws_config(&self) -> &aws::SdkConfig {
        &self.aws_config
    }
}

impl HasDynamoDBConfiguration for DynamoDBConfiguration {
    fn dynamodb_table(&self) -> &str {
        &self.table
    }

    fn dynamodb_partition_key(&self) -> &str {
        &self.partition_key
    }

    fn dynamodb_ttl(&self) -> tokio::time::Duration {
        self.ttl
    }
}
