//! Helper functions to confirm that the user has logged in with the correct credentials.
//!

use crate::CoffeeShopError;

pub use aws_sdk_sts::operation::get_caller_identity::GetCallerIdentityOutput as AWSCallerIdentity;

use super::aws;

#[cfg(feature = "debug")]
const LOG_TARGET: &str = "coffeeshop::helpers::sts";

/// Confirm that the user has logged in with the correct credentials.
pub async fn get_aws_login(
    config: Option<&aws_config::SdkConfig>,
) -> Result<AWSCallerIdentity, CoffeeShopError> {
    // Extract the configuration or read it from the environment.
    let config = if let Some(config) = config {
        config
    } else {
        &aws::get_aws_config().await?
    };

    crate::trace!(
        target: LOG_TARGET,
        "Attempting to get STS caller identity with configuration: {:?}",
        config
    );

    let client = aws_sdk_sts::Client::new(config);

    // Check that the region is set, otherwise STS will return a very unhelpful
    // "dispatch failure" message, which cannot be easily checked due to `Unhandled`
    // enum variant.
    if config.region().is_none() {
        return Err(CoffeeShopError::AWSConfigIncomplete(
            "AWS Region is not set.".to_string(),
        ));
    }

    client
        .get_caller_identity()
        .send()
        .await
        .map_err(|sdk_err| {
            CoffeeShopError::from_aws_sts_error(sdk_err.into_service_error().into(), config)
        })
}

/// Report the AWS caller identity.
pub async fn report_aws_login(
    config: Option<&aws_config::SdkConfig>,
) -> Result<(), CoffeeShopError> {
    let identity = get_aws_login(config).await?;

    // TODO Do something if the identity is not as expected?
    crate::info!(
        target: LOG_TARGET,
        "AWS credentials: UserId: {:?}, Account: {:?}, Arn: {:?}",
        identity.user_id.unwrap_or("(none)".to_string()),
        identity.account.unwrap_or("(none)".to_string()),
        identity.arn.unwrap_or("(none)".to_string()),
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[cfg(feature = "test_on_aws")]
    async fn get_aws_login_with_default() {
        let result = get_aws_login(None).await.inspect(|result| {
            crate::info!(
                target: LOG_TARGET,
                "Received AWS caller identity: {:?}",
                result
            )
        });
        assert!(result.is_ok());
    }
}
