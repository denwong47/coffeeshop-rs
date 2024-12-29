use crate::{
    helpers::sqs,
    models::{message, Machine},
};
use serde::{de::DeserializeOwned, Serialize};

use super::Shop;

impl<Q, I, O, F> sqs::HasSQSConfiguration for Shop<Q, I, O, F>
where
    Q: message::QueryType,
    I: Serialize + DeserializeOwned,
    O: Serialize + DeserializeOwned + Send + Sync,
    F: Machine<Q, I, O>,
{
    /// The SQS queue URL for the shop.
    fn sqs_queue_url(&self) -> &str {
        &self.sqs_queue
    }
}
