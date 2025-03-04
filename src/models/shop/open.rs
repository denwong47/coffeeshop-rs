use super::*;

use std::sync::Arc;
use tokio::sync::Notify;

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    helpers,
    models::{message, Barista, Machine},
    CoffeeShopError,
};

const LOG_TARGET: &str = "coffeeshop::models::shop";

/// The interval to check for fulfilled orders in DynamoDB.
const CHECK_DYNAMODB_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_secs(15);

impl<Q, I, O, F> Shop<Q, I, O, F>
where
    Q: message::QueryType + 'static,
    I: Serialize + DeserializeOwned + Send + Sync + 'static,
    O: Serialize + DeserializeOwned + Send + Sync + 'static,
    F: Machine<Q, I, O> + 'static,
{
    /// Open the shop, start listening for requests.
    ///
    /// This function will start the waiter, baristas, and announcer.
    ///
    /// # Parameters
    ///
    /// - `shutdown_signal` - A signal to shutdown the shop. This will be used internally
    ///   to gracefully shutdown the shop. You can also use this signal to shutdown the shop
    ///   from the outside.
    /// - `additional_routes` - Additional routes to be added to the waiter. This is useful
    ///   when you want to add custom routes to the waiter. If you do not want to add any,
    ///   pass a `vec![].into_iter()`.
    pub async fn open(
        &self,
        shutdown_signal: Option<Arc<Notify>>,
        additional_routes: impl Iterator<
            Item = (
                &'static str,
                axum::routing::method_routing::MethodRouter<()>,
            ),
        >,
    ) -> Result<(), CoffeeShopError> {
        // If the shutdown signal is not provided, create a new one.
        let shutdown_signal = shutdown_signal.unwrap_or_else(|| Arc::new(Notify::new()));

        // Report the AWS login status in order to confirm the AWS credentials.
        helpers::sts::report_aws_login(Some(&self.aws_config)).await?;

        let max_execution_time = self.config.max_execution_time();

        // Using join instead of select to allow all tasks to gracefully shutdown.
        tokio::try_join! {
            // Termination signal.
            async {
                tokio::select!(
                    _ = tokio::signal::ctrl_c() => {
                        crate::warn!(target: LOG_TARGET, "Received termination signal. Shutting down the shop.");
                        shutdown_signal.clone().notify_waiters();
                    },
                    _ = shutdown_signal.notified() => {
                        crate::warn!(target: LOG_TARGET, "A 3rd party had requested shutdown; stop listening for SIGTERM.");
                    },
                );

                Ok::<(), CoffeeShopError>(())
            },
            // Waiter.
            async {
                self.waiter.serve(additional_routes, shutdown_signal.clone(), max_execution_time).await
                .inspect_err(
                    |err| crate::error!(target: LOG_TARGET, "The waiter has stopped serving requests. Error: {:?}", err)
                )
            },
            // Baristas.
            async {
                Barista::serve_all(&self.baristas, shutdown_signal.clone()).await
                .inspect_err(
                    |err| crate::error!(target: LOG_TARGET, "The baristas have stopped serving requests. Error: {:?}", err)
                )
            },
            // Announcer.
            async {
                // Announce the shop is now opening.
                // Since the announcer already bound its sockets, it will in fact hear this announcement.
                self.announcer.announce_status(message::MulticastMessageStatus::Success).await?;
                let result = self.announcer.listen_for_announcements(shutdown_signal.clone()).await
                    .inspect_err(
                        |err| crate::error!(target: LOG_TARGET, "The announcer has stopped listening for announcements. Error: {:?}", err)
                    );

                // Announce the shop is now closing.
                self.announcer.announce_status(
                    {
                        if result.is_ok() {
                            message::MulticastMessageStatus::Aborted
                        } else {
                            message::MulticastMessageStatus::Error
                        }
                    }
                ).await?;

                result
            },
            // Shop periodic checking of DynamoDB as a final line of defence.
            async {
                self.periodically_check_for_fulfilled_orders(CHECK_DYNAMODB_INTERVAL, shutdown_signal.clone()).await
                .inspect_err(
                    |err| crate::error!(target: LOG_TARGET, "The shop has stopped checking for fulfilled orders. Error: {:?}", err)
                )
            },
        }
        .map(|_|
            crate::info!(target: LOG_TARGET, "The shop has been closed. See you!")
        )
        .inspect_err(
            |_|
                crate::error!(target: LOG_TARGET, "The shop has been shutdown due to the above error.")
        )
    }
}
