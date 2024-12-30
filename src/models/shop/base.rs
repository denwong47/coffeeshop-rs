use hashbrown::HashMap;
use serde::{de::DeserializeOwned, Serialize};
use std::{marker::PhantomData, sync::Arc};
use tokio::sync::RwLock;

use super::super::{message, Barista, Machine, Order, Orders, Ticket, Waiter};
use crate::{cli::Config, helpers, CoffeeShopError};

/// The default prefix for dynamodb table.
const DYNAMODB_TABLE_PREFIX: &str = "task-queue-";

/// The default prefix for SQS queue.
const SQS_QUEUE_PREFIX: &str = "task-queue-";

/// A coffee shop that has a waiter to take orders, and positive number of baristas to process
/// tickets using the coffee machine.
///
/// The shop is expected to:
/// - Listen for incoming requests,
/// - Convert the requests into tickets on a shared AWS SQS queue,
/// - Have baristas to process the tickets using the coffee machine,
/// - Put the finished coffee into a AWS DynamoDB table using the SQS id as the key, then
/// - The barista will shout out the ticket number for the waiter to pick up the order.
///
/// The [`Shop`] is designed to work with load balancers and auto-scaling groups, so that more
/// [`Shop`] instances can be deployed to the same cluster to handle the same
/// queue, without dropping any messages. The load balancing can be performed on the
/// number of messages in the queue.
///
/// Depending on the node type for the [`Shop`], each
/// [`Shop`] can have a different number of baristas within it, but will always have one
/// waiter. Choosing the waiter to serve incoming requests is the responsibility of the
/// load balancer, and is not part of this implementation; however as the waiter has
/// very virtually no blocking work to do, [`tokio`] alone should be able to handle
/// a large number of requests even if they are not perfectly balanced across [`Shop`]s.
///
/// # Note
///
/// One part where this analogy breaks down is that the customer could be directed to
/// any [`Shop`] in the cluster to place an order, but if he chooses not to wait for
/// the order to be ready, he will end up picking up the order from a different [`Shop`]
/// than the one he ordered, and perhaps even a different one to the one that made the
/// coffee.
///
/// This can possibly be solved by making the Application Load Balancer sticky, so that
/// the customer is always directed to the same [`Shop`] to pick up the order; but this
/// is not necessary in practice.
///
/// Perhaps the problem is with the real world - why shouldn't Starbucks be able to
/// do that?
#[derive(Debug)]
pub struct Shop<Q, I, O, F>
where
    Q: message::QueryType,
    I: Serialize + DeserializeOwned + Send + Sync,
    O: Serialize + DeserializeOwned + Send + Sync,
    F: Machine<Q, I, O>,
{
    /// The name of the task that this shop is responsible for.
    ///
    /// This is used to ensure multicast messages are only processed by the correct shop.
    /// Ideally, each shop should use unique multicast addresses to prevent message collisions.
    pub name: String,

    /// A map of tickets to their respective [`Notify`] events that are used to notify the
    /// waiter when a ticket is ready.
    pub orders: RwLock<Orders>,

    /// The coffee machine that will process tickets.
    ///
    /// This is the actual task that will be executed when a ticket is received. It should be able
    /// to tell apart any different types of tickets among the generic input type `I`, and produce
    /// a generic output type `O` regardless of the input type.
    pub coffee_machine: F,

    /// Dynamodb table name to store the finished products.
    pub dynamodb_table: String,

    /// The SQS queue name to store the tickets.
    pub sqs_queue: String,

    /// The configuration for the shop.
    ///
    /// These include the settings for the multicast address, the port, and the IP address, number
    /// of baristas etc.
    pub config: Config,

    /// The AWS SDK configuration for the shop.
    pub aws_config: helpers::aws::SdkConfig,

    /// Temporary Directory for serialization and deserialization.
    pub(crate) temp_dir: tempfile::TempDir,

    /// Reference to the waiter that will serve incoming requests.
    pub waiter: Waiter<Q, I, O, F>,

    /// Reference to the baristas that will process the tickets.
    pub baristas: Vec<Barista<Q, I, O, F>>,

    /// Phantom data to attach the input and output types to the shop.
    _phantom: PhantomData<(Q, I, O)>,
}

impl<Q, I, O, F> Shop<Q, I, O, F>
where
    Q: message::QueryType,
    I: Serialize + DeserializeOwned + Send + Sync,
    O: Serialize + DeserializeOwned + Send + Sync,
    F: Machine<Q, I, O>,
{
    /// Create a new shop with the given name, coffee machine, and configuration.
    pub async fn new(
        name: String,
        coffee_machine: F,
        mut config: Config,
        aws_config: Option<helpers::aws::SdkConfig>,
        barista_count: usize,
    ) -> Result<Arc<Self>, CoffeeShopError> {
        // If the table has not been set, use the default table name with the prefix.
        // Otherwise, remove the name from `config` and put it into the [`Shop`].
        let dynamodb_table = config
            .dynamodb_table
            .take()
            .unwrap_or_else(|| format!("{}{}", DYNAMODB_TABLE_PREFIX, &name));

        let sqs_queue = config
            .sqs_queue
            .take()
            .unwrap_or_else(|| format!("{}{}", SQS_QUEUE_PREFIX, &name));

        let aws_config = if let Some(aws_config) = aws_config {
            aws_config
        } else {
            helpers::aws::get_aws_config().await?
        };

        let temp_dir = tempfile::TempDir::new()
            .map_err(|err| CoffeeShopError::TempDirCreationFailure(err.to_string()))?;
        Ok(Arc::new_cyclic(|me| Self {
            name,
            orders: HashMap::new().into(),
            coffee_machine,
            dynamodb_table,
            sqs_queue,
            config,
            aws_config,
            temp_dir,
            waiter: Waiter::new(me.clone()),
            baristas: (0..barista_count)
                .map(|_| Barista::new(me.clone()))
                .collect::<Vec<Barista<Q, I, O, F>>>(),
            _phantom: PhantomData,
        }))
    }

    /// Open the shop, start listening for requests.
    pub async fn open(&self) -> Result<(), CoffeeShopError> {
        // Report the AWS login status in order to confirm the AWS credentials.
        helpers::sts::report_aws_login(Some(&self.aws_config)).await?;

        unimplemented!()
    }

    /// Spawn a [`Order`] order for a given [`Ticket`] in the shop.
    ///
    /// Get the ticket if it exists, otherwise create a new one
    /// before returning the [`Arc`] reference to the [`Order`].
    pub async fn spawn_order(&self, ticket: Ticket) -> Arc<Order> {
        self.orders
            .write()
            .await
            .entry(ticket.clone())
            .or_insert_with_key(|_| Arc::new(Order::new(ticket)))
            .clone()
    }
}
