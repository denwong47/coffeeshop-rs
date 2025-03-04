//! Unified interface for the creation of sockets, and low-level multicast operations.

use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::CoffeeShopError;

const LOG_TARGET: &str = "coffeeshop::helpers::multicast::socket";

use super::AsyncSocket;

/// A helper function to describe a [`SockAddr`].
///
/// This is distinct from [`describe_socket_addr`] which is the [`std::net`] equivalent.
pub fn describe_sock_addr(sock_addr: &SockAddr) -> String {
    sock_addr
        .as_socket()
        .map(|sock_addr| describe_socket_addr(&sock_addr))
        .unwrap_or_else(|| "(Unknown source)".to_owned())
}

/// A helper function to describe a [`SocketAddr`].
///
/// This is distinct from [`describe_sock_addr`] which is the [`socket2`] equivalent.
pub fn describe_socket_addr(socket_addr: &SocketAddr) -> String {
    format!(
        "{ip}:{port}",
        ip = socket_addr.ip(),
        port = socket_addr.port()
    )
}

/// Create a generic UDP socket that can be used for multicast communication.
///
/// The resultant socket can be used for both sending and receiving multicast messages.
///
/// By default, the socket will be:
/// - non-blocking,
/// - allow the reuse of the address, and
/// - bound to the given address.
pub fn create_udp(addr: &SocketAddr) -> Result<AsyncSocket, CoffeeShopError> {
    let builder = || {
        let domain = Domain::for_address(*addr);
        let socket = Socket::new(domain, Type::DGRAM, Some(Protocol::UDP))?;

        socket.set_nonblocking(true)?;
        socket.set_reuse_address(true)?;
        socket.bind(&SockAddr::from(*addr))?;

        AsyncSocket::new(socket)
    };

    builder().map_err(CoffeeShopError::from_multicast_io_error)
}

/// A short hand function to create a UDP socket bound to all IPv4 interfaces.
///
/// This is useful for creating a sender socket.
pub fn create_udp_all_v4_interfaces(port: u16) -> Result<AsyncSocket, CoffeeShopError> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    create_udp(&addr)
}

/// A helper function to set a socket to join a multicast address.
///
/// The resultant socket will listen for multicast messages on all interfaces.
pub fn join_multicast(asocket: &AsyncSocket, addr: &SocketAddr) -> Result<(), CoffeeShopError> {
    let socket = asocket.get_ref();

    let ip_addr = addr.ip();

    if !ip_addr.is_multicast() {
        return Err(CoffeeShopError::InvalidConfiguration {
            field: "multicast_host",
            message: format!("Address {ip_addr} is not a multicast address"),
        });
    }

    // This block creates an io::Error; needs to map it to a CoffeeShopError.
    match ip_addr {
        IpAddr::V4(ref mdns_v4) => socket.join_multicast_v4(mdns_v4, &Ipv4Addr::new(0, 0, 0, 0)),
        IpAddr::V6(ref mdns_v6) => {
            // This does not work on macOS which REQUIRES an interface to be specified.
            socket
                .join_multicast_v6(mdns_v6, 0)
                .and_then(|_| socket.set_only_v6(true))
        }
    }
    .map_err(CoffeeShopError::from_multicast_io_error)
}

/// A helper function to send a multicast message.
pub async fn send_multicast(
    asocket: &AsyncSocket,
    addr: &SocketAddr,
    data: &[u8],
) -> Result<usize, CoffeeShopError> {
    crate::debug!(
        target: LOG_TARGET,
        "Sending {} bytes to {:?}...",
        data.len(),
        describe_sock_addr(&SockAddr::from(*addr))
    );
    asocket
        .write(|socket| socket.send_to(data, &SockAddr::from(*addr)))
        .await
        .map_err(CoffeeShopError::from_multicast_io_error)
}

/// A helper function to receive a multicast message.
pub async fn receive_multicast(
    asocket: &AsyncSocket,
    buffer_size: usize,
) -> Result<(Vec<u8>, SockAddr), CoffeeShopError> {
    let mut inner_buffer = vec![core::mem::MaybeUninit::uninit(); buffer_size];
    crate::debug!(target: LOG_TARGET, "Waiting for message...");
    let result = asocket
        .read(|socket| socket.recv_from(&mut inner_buffer))
        .await;
    crate::trace!(target: LOG_TARGET, "Received message.");

    result
        .map(|(size, addr)| {
            crate::debug!(
                target: LOG_TARGET,
                "Received {} bytes from {:?}.",
                size,
                describe_sock_addr(&addr)
            );

            // Only take the initialized part of the buffer.
            (
                (0..size)
                    .map(|i| unsafe { inner_buffer[i].assume_init() })
                    .collect::<Vec<_>>(),
                addr,
            )
        })
        .map_err(CoffeeShopError::from_multicast_io_error)
}

/// Do not run these tests in CI.
#[cfg(test)]
// #[cfg(not(feature = "test_on_ci"))]
mod test {
    use super::super::test;
    use super::*;
    use std::io;

    use serial_test::serial;

    const LOG_TARGET: &str = "coffeeshop::helpers::multicast::socket::test";
    const REPEAT: usize = 3;

    #[tokio::test]
    #[serial]
    async fn hello_world() {
        let addr = test::get_multicast_addr();

        let sender = create_udp_all_v4_interfaces(0).expect("Failed to create sender socket!");
        let receiver = create_udp(&addr).expect("Failed to create receiver socket!");
        join_multicast(&receiver, &addr).expect("Failed to join multicast group!");

        let listener = tokio::spawn(async move {
            let mut received = Vec::with_capacity(REPEAT);

            for id in 0..REPEAT {
                let (data, _addr) = receive_multicast(&receiver, 1024)
                    .await
                    .expect("Failed to receive multicast message!");
                crate::debug!(
                    target: LOG_TARGET,
                    "Received message #{} from {:?}: {:?}",
                    id,
                    describe_sock_addr(&_addr),
                    data
                );

                received.push(data);
            }

            received
        });

        tokio::time::sleep(tokio::time::Duration::from_nanos(50)).await;

        let data = b"Hello, world!";

        for id in 0..REPEAT {
            let _sent = send_multicast(&sender, &addr, data)
                .await
                .expect("Failed to send multicast message!");
            crate::debug!(target: LOG_TARGET, "Sent {} bytes in message #{}.", _sent, id);
        }

        // Wait for the listener to finish, with a timeout.
        // Convert all errors to io::Error; we could use CoffeeShopError, but it's a test,
        // lets not complicate things.
        for received_data in tokio::time::timeout(tokio::time::Duration::from_secs(1), listener)
            .await
            .map_err(|timeout| io::Error::new(io::ErrorKind::TimedOut, timeout))
            .and_then(|result| {
                result.map_err(|join_error| {
                    io::Error::new(io::ErrorKind::Other, join_error.to_string())
                })
            })
            .expect("Failed to receive messages!")
        {
            assert_eq!(data, received_data.as_slice());
            crate::debug!(target: LOG_TARGET, "Received message passed assertion.")
        }
    }
}
