use super::{message, Machine, Shop};
use prost::Message;
use serde::{de::DeserializeOwned, Serialize};
use socket2::SockAddr;
use std::sync::{Arc, OnceLock, Weak};
use tokio::sync::Notify;

use crate::{helpers::multicast, CoffeeShopError};

/// The default buffer size for receiving multicast messages.
const DEFAULT_BUFFER_SIZE: usize = 1024;

const LOG_TARGET: &str = "coffeeshop::models::announcer";

/// An [`Announcer`] is a person who broadcasts the orders that are ready to other
/// [`Announcer`]s in other [`Shop`]s.
pub struct Announcer<Q, I, O, F>
where
    Q: message::QueryType,
    I: Serialize + DeserializeOwned + Send + Sync,
    O: Serialize + DeserializeOwned + Send + Sync,
    F: Machine<Q, I, O>,
{
    // TODO consider making this a Generic with a HasMulticastConfiguration trait, so that
    // we can skip the 4 type parameters here.
    shop: Weak<Shop<Q, I, O, F>>,
    sender: OnceLock<multicast::AsyncSocket>,
    receiver: OnceLock<multicast::AsyncSocket>,
}

impl<Q, I, O, F> std::fmt::Debug for Announcer<Q, I, O, F>
where
    Q: message::QueryType,
    I: Serialize + DeserializeOwned + Send + Sync,
    O: Serialize + DeserializeOwned + Send + Sync,
    F: Machine<Q, I, O>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Announcer")
            .field("shop", &self.shop)
            // Skip the sender and receiver fields because they are not useful for debugging.
            .finish()
    }
}

impl<Q, I, O, F> Announcer<Q, I, O, F>
where
    Q: message::QueryType + 'static,
    I: Serialize + DeserializeOwned + Send + Sync + 'static,
    O: Serialize + DeserializeOwned + Send + Sync + 'static,
    F: Machine<Q, I, O>,
{
    /// Create a new, uninitialized announcer with the given shop.
    ///
    /// We could not complete the initialization in the constructor because
    /// the shop provided here is a cyclical reference that would not have been
    /// created yet, so none of the multicast parameters would have been
    /// available.
    ///
    /// Call [`init`](Self::init) after [`Shop`] is initialized to complete the
    /// initialization; this step is typically done in the [`Shop::new`] constructor.
    pub fn new(shop: Weak<Shop<Q, I, O, F>>) -> Self {
        Self {
            shop,
            sender: OnceLock::new(),
            receiver: OnceLock::new(),
        }
    }

    /// Get the multicast address that this announcer is serving.
    fn multicast_addr(&self) -> std::net::SocketAddr {
        self.shop().config.multicast_addr()
    }

    /// Initialize the multicast socket for sending.
    ///
    /// The socket for sending is bound to all IPv4 interfaces, and is not active
    /// until a message is sent. It is stateless; hence we only need one for each
    /// [`Announcer`] instance.
    fn init_sender(&self) -> Result<multicast::AsyncSocket, CoffeeShopError> {
        let addr = self.multicast_addr();

        multicast::socket::create_udp_all_v4_interfaces(0).inspect_err(|err| {
            crate::error!(
                target: LOG_TARGET,
                "Failed to create multicast sender socket at {addr:?}: {err}",
                addr = &addr,
                err = err
            )
        })
    }

    /// Initialize the multicast socket for listening.
    ///
    /// # Note
    ///
    /// In the future, once [once_cell_try](https://github.com/rust-lang/rust/issues/109737)
    /// is stabilized, then this function can have a return type of `Result`.
    fn init_receiver(&self) -> Result<multicast::AsyncSocket, CoffeeShopError> {
        let addr = self.multicast_addr();

        multicast::socket::create_udp(&addr)
            .inspect_err(|err| {
                crate::error!(
                    target: LOG_TARGET,
                    "Failed to create multicast socket at {addr:?}: {err}",
                    addr = &addr,
                    err = err
                )
            })
            .and_then(|asocket| {
                multicast::socket::join_multicast(&asocket, &addr)
                    .inspect_err(|err| {
                        crate::error!(
                            target: LOG_TARGET,
                            "Failed to join multicast group at {addr:?}: {err}",
                            addr = &addr,
                            err = err
                        )
                    })
                    .map(|_| asocket)
            })
    }

    /// Initialize the [`Announcer`].
    ///
    /// Compared to not initializing the [`Announcer`] and using it directly, this
    /// provides each of the initialization steps a chance to fail, and the error can
    /// be gracefully handled.
    ///
    /// Using this method straight after [`Shop`] is initialized is strongly
    /// recommended. This should be done for you in the [`Shop::new`] constructor.
    ///
    /// # Safety
    ///
    /// This method is safe to call only if no other references to the [`Announcer`]
    /// exist; this is due to a time-of-check-time-of-use (TOCTOU) situation where
    /// the sender and receiver may be initialized by another thread between the
    /// check and the initialization.
    ///
    /// In such a case, this method may return an error about Multicast Sockets not
    /// being initialized. In the case where double initialization may have occurred,
    /// this should not be a problem, as the second initialization will be a no-op; but
    /// it is still recommended to avoid this situation.
    pub fn init(&self) -> Result<(), CoffeeShopError> {
        // This pattern is only safe if `self` is owned!
        // Do not copy this pattern for other types.
        if self.sender.get().is_none() {
            drop(self.sender.set(self.init_sender()?));
        }
        if self.receiver.get().is_none() {
            drop(self.receiver.set(self.init_receiver()?));
        }

        Ok(())
    }

    /// Get the back reference to the shop that this announcer is serving.
    pub fn shop(&self) -> Arc<Shop<Q, I, O, F>> {
        self.shop.upgrade()
        .expect("Shop has been dropped; this should not be possible in normal use. Please report this to the maintainer.")
    }

    /// Get the multicast socket for sending.
    pub fn sender(&self) -> &multicast::AsyncSocket {
        // A fallback if the sender is not initialized.
        // This may be a bad idea; let's revisit this later.
        self.sender.get_or_init(|| {
            self.init_sender()
                .expect("Failed to initialize multicast sender.")
        })
    }

    /// Get the multicast socket for receiving.
    pub fn receiver(&self) -> &multicast::AsyncSocket {
        // A fallback if the sender is not initialized.
        // This may be a bad idea; let's revisit this later.
        self.receiver.get_or_init(|| {
            self.init_receiver()
                .expect("Failed to initialize multicast receiver.")
        })
    }

    /// Send a message to all announcers in the multicast group.
    ///
    /// This function will encode the message and send it to the multicast group.
    ///
    /// # Returns
    ///
    /// The number of bytes sent.
    pub async fn send_message(
        &self,
        msg: message::MulticastMessage,
    ) -> Result<usize, CoffeeShopError> {
        let encoded = msg.encode_to_vec();

        multicast::socket::send_multicast(self.sender(), &self.multicast_addr(), &encoded)
            .await
            .inspect_err(|err| {
                crate::error!(
                    target: LOG_TARGET,
                    "Failed to send multicast message: {err}",
                    err = err
                )
            })
    }

    /// Static method to transform a received message into a [`MulticastMessage`].
    fn transform_message(
        &self,
        data: Vec<u8>,
        addr: SockAddr,
    ) -> Result<message::MulticastMessage, CoffeeShopError> {
        message::MulticastMessage::decode(&data[..])
            .inspect_err(|err| {
                crate::error!(
                    target: LOG_TARGET,
                    "Failed to decode multicast message from {addr:?}: {err}",
                    addr = &addr,
                    err = err
                )
            })
            .map_err(|err| CoffeeShopError::InvalidMulticastMessage {
                data,
                addr: multicast::socket::describe_sock_addr(&addr),
                error: err,
            })
    }

    /// Receive a message from the multicast group, transform it into a [`MulticastMessage`],
    /// and handle it according to its message type.
    ///
    /// Internal function, meant to be called by the [`listen_for_announcements`] function.
    async fn received_message_handler(
        &self,
        data: Vec<u8>,
        addr: SockAddr,
    ) -> Result<(), CoffeeShopError> {
        let message = self.transform_message(data, addr)?;

        crate::info!(
            target: LOG_TARGET,
            "Received multicast message: {message:?}",
            message = &message
        );

        match (message.kind(), message.status()) {
            // If the message is a ticket that has been completed or rejected, then
            // the processing is finished and we can log it into the shop.
            (message::MulticastMessageKind::Ticket, status) if status.is_finished() => {
                let shop = self.shop();

                if let Some(order) = shop.get_order(&message.ticket).await {
                    order
                        .value()
                        .complete(status == message::MulticastMessageStatus::Complete)
                        .inspect_err(|err| {
                            crate::error!(
                                target: LOG_TARGET,
                                "Failed to set order {ticket:?} to complete, ignoring: {err}",
                                ticket = &message.ticket,
                                err = err
                            )
                        })?;
                } else {
                    crate::info!(
                        target: LOG_TARGET,
                        "Received completion message for irrelevant ticket {ticket:?}, ignoring.",
                        ticket = &message.ticket
                    )
                }
            }
            (kind, status) => {
                crate::info!(
                    target: LOG_TARGET,
                    "Received irrelevant multicast message kind and status, ignored: {kind:?}, {status:?}",
                );
            }
        }

        Ok(())
    }

    /// Listen for announcements from other [`Announcer`]s as well as itself.
    pub async fn listen_for_announcements(
        &self,
        shutdown_signal: Arc<Notify>,
    ) -> Result<(), CoffeeShopError> {
        let mut message_count: u64 = 0;

        // TODO Every once in a while, we should check with DynamoDB regardless of the multicast messages.

        // This is the main task that listens for multicast messages, to be raced against the shutdown signal.
        let task = async {
            loop {
                if let Ok((data, addr)) =
                    multicast::socket::receive_multicast(self.receiver(), DEFAULT_BUFFER_SIZE)
                        .await
                        .inspect_err(|err| {
                            crate::error!(
                                target: LOG_TARGET,
                                "Failed to receive multicast message, skipping: {err}",
                                err = err
                            )
                        })
                {
                    if self.received_message_handler(data, addr).await.is_ok() {
                        crate::info!(
                            target: LOG_TARGET,
                            "Processed multicast message #{message_count} successfully."
                        );
                    } else {
                        crate::error!(
                            target: LOG_TARGET,
                            "Failed to process multicast message #{message_count}, skipping."
                        )
                    }
                } else {
                    crate::error!(
                        target: LOG_TARGET,
                        "Failed to receive multicast message, skipping."
                    )
                }

                message_count = message_count.wrapping_add(1);
            }
        };

        tokio::select! {
            _ = shutdown_signal.notified() => {
                crate::warn!(target: LOG_TARGET, "Received shutdown signal, terminating announcer.");
                Ok(())
            },
            result = task => {
                crate::error!(target: LOG_TARGET, "Failed to listen for announcements: {result:?}");
                result
            },
        }
    }
}
