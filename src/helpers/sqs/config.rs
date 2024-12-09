use crate::helpers::aws::{self, HasAWSSdkConfig};
use std::sync::Arc;

#[cfg(doc)]
use crate::models::Shop;

/// A [`HasSQSConfiguration`] contains the configuration for the DynamoDB table
/// that the [`Shop`] will be using.
pub trait HasSQSConfiguration: HasAWSSdkConfig {
    /// The name of the SQS table.
    fn sqs_queue_url(&self) -> &str;

    /// Extract the configuration as a separate struct.
    ///
    /// This is useful if the main configuration struct is too large, or it
    /// lacks certain traits such as [`Send`] or [`Sync`].
    fn sqs_configuration(&self) -> SQSConfiguration {
        SQSConfiguration {
            queue_url: self.sqs_queue_url().to_owned(),
            aws_config: self.aws_config().clone(),
        }
    }
}

/// A minimal implementation of [`SQSConfiguration`] for testing purposes, or
/// to use this module without a full [`Shop`] configuration.
pub struct SQSConfiguration {
    pub queue_url: String,
    pub aws_config: aws::SdkConfig,
}

impl HasAWSSdkConfig for SQSConfiguration {
    fn aws_config(&self) -> &aws::SdkConfig {
        &self.aws_config
    }
}

impl HasSQSConfiguration for SQSConfiguration {
    fn sqs_queue_url(&self) -> &str {
        &self.queue_url
    }
}

impl<T> HasAWSSdkConfig for Arc<T>
where
    T: HasAWSSdkConfig,
{
    fn aws_config(&self) -> &aws::SdkConfig {
        (**self).aws_config()
    }
}

impl<T> HasSQSConfiguration for Arc<T>
where
    T: HasSQSConfiguration,
{
    fn sqs_queue_url(&self) -> &str {
        (**self).sqs_queue_url()
    }
}
