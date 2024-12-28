use crate::{
    helpers::aws,
    models::{message, Machine},
};
use serde::{de::DeserializeOwned, Serialize};

use super::Shop;

impl<Q, I, O, F> aws::HasAWSSdkConfig for Shop<Q, I, O, F>
where
    Q: message::QueryType,
    I: Serialize + DeserializeOwned,
    O: Serialize + DeserializeOwned,
    F: Machine<Q, I, O>,
{
    /// The AWS SDK configuration for the shop.
    fn aws_config(&self) -> &aws::SdkConfig {
        &self.aws_config
    }
}
