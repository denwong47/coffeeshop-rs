//! Centralized AWS helper functions.

use crate::CoffeeShopError;

/// Re-export the AWS configuration.
pub use aws_config::SdkConfig;

/// Get the AWS configuration from the environment variables or the provided arguments.
///
/// # Note
///
/// Currently this function only reads the configuration from the environment variables, and
/// is always successful; however, in the future, it may be extended to read from a configuration
/// file or other sources, which could fail.
pub async fn get_aws_config() -> Result<aws_config::SdkConfig, CoffeeShopError> {
    let config = aws_config::load_from_env().await;

    Ok(config)
}

/// A trait indicating that the implementing struct has an AWS SDK configuration.
pub trait HasAWSSdkConfig: Send + Sync {
    /// Get the AWS configuration.
    fn aws_config(&self) -> &SdkConfig;
}
