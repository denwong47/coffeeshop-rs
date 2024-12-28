use crate::helpers::aws::{self, HasAWSSdkConfig};

#[cfg(doc)]
use crate::models::Shop;

/// A [`HasSQSConfiguration`] contains the configuration for the DynamoDB table
/// that the [`Shop`] will be using.
pub trait HasSQSConfiguration: HasAWSSdkConfig {
    /// The name of the SQS table.
    fn sqs_queue_url(&self) -> &str;
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
