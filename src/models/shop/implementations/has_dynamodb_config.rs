use crate::{
    helpers::dynamodb,
    models::{message, Machine},
};
use serde::{de::DeserializeOwned, Serialize};

use super::Shop;

impl<Q, I, O, F> dynamodb::HasDynamoDBConfiguration for Shop<Q, I, O, F>
where
    Q: message::QueryType,
    I: Serialize + DeserializeOwned,
    O: Serialize + DeserializeOwned + Send + Sync,
    F: Machine<Q, I, O>,
{
    fn dynamodb_table(&self) -> &str {
        &self.dynamodb_table
    }

    fn dynamodb_partition_key(&self) -> &str {
        &self.config.dynamodb_partition_key
    }

    /// The time-to-live (TTL) duration for the items in the DynamoDB table.
    fn dynamodb_ttl(&self) -> tokio::time::Duration {
        self.config.dynamodb_ttl()
    }
}
