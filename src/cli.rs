use clap::Parser;

use crate::CoffeeShopError;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

/// The default host address for the Waiter, which is to listen on all interfaces.
const DEFAULT_HOST: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);

/// The default port for the Waiter, which is `7007`.
const DEFAULT_PORT: u16 = 7007;

/// The default multicast address for the Announcer.
const MULTICAST_HOST: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 249);

/// The default port for the Announcer, which is `65355`.
const MULTICAST_PORT: u16 = 65355;

/// The number of Baristas to initiate.
const DEFAULT_BARISTAS: u16 = 1;

/// The default partition key (Primary Key) to use with the DynamoDB Table.
///
/// This must be set to match the table's partition key.
const DEFAULT_DYNAMODB_PARTITION_KEY: &str = "identifier";

/// The default TTL for the results in seconds.
const DEFAULT_RESULT_TTL: f32 = 7200.;

/// The maximum number of outstanding tickets before the waiter starts rejecting new
/// requests with a `429 Too Many Requests` status code.
const MAX_TICKETS: usize = 1024;

/// Simple program to greet a person
#[derive(Parser, Debug, PartialEq)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// The host IP of the server. Defaults to all interfaces.
    #[arg(long, default_value_t = DEFAULT_HOST)]
    pub host: Ipv4Addr,

    /// The port to listen on. Defaults to [`DEFAULT_PORT`].
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    pub port: u16,

    /// The address to listen for Multicast.
    #[arg(long, default_value_t = MULTICAST_HOST)]
    pub multicast_host: Ipv4Addr,

    /// The port to listen for Multicast.
    #[arg(long, default_value_t = MULTICAST_PORT)]
    pub multicast_port: u16,

    /// The number of Baristas to initiate.
    #[arg(long, default_value_t = DEFAULT_BARISTAS, alias = "workers")]
    pub baristas: u16,

    /// Maximum number of outstanding tickets.
    #[arg(long, default_value_t = MAX_TICKETS)]
    pub max_tickets: usize,

    /// The AWS DynamoDB table to use.
    #[arg(long, default_value = None)]
    pub dynamodb_table: Option<String>,

    /// The partition key to use with the DynamoDB table.
    #[arg(long, default_value = DEFAULT_DYNAMODB_PARTITION_KEY, alias = "dynamodb_primary_key")]
    pub dynamodb_partition_key: String,

    /// The number of seconds to keep the results in the DynamoDB table before it can
    /// get purged by AWS.
    #[arg(long, default_value_t = DEFAULT_RESULT_TTL)]
    pub result_ttl: f32,

    /// The maximum time a ticket can be processed before it is killed by the
    /// HTTP server.
    #[arg(long, default_value = None)]
    pub max_execution_time: Option<f32>,

    /// The AWS SQS queue URL to use.
    ///
    /// The AWS user must have the necessary permissions to send and receive messages
    /// from this queue
    #[arg(long, default_value = None)]
    pub sqs_queue: Option<String>,
}

impl Default for Config {
    /// Get the args with the default configuration.
    ///
    /// This allows [`Config`] to be used without parsing the CLI args. This is useful
    /// when this framework is not implemented as a CLI tool, and the configurations are
    /// sourced from elsewhere.
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST,
            port: DEFAULT_PORT,
            multicast_host: MULTICAST_HOST,
            multicast_port: MULTICAST_PORT,
            baristas: DEFAULT_BARISTAS,
            max_tickets: MAX_TICKETS,
            dynamodb_table: None,
            dynamodb_partition_key: DEFAULT_DYNAMODB_PARTITION_KEY.to_owned(),
            result_ttl: DEFAULT_RESULT_TTL,
            max_execution_time: None,
            sqs_queue: None,
        }
    }
}

impl Config {
    /// Instantiate a new [`Config`] with [`Self::default`] settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if the multicast address is correct; if not, consume itself and
    /// return an [`Err`].
    fn validate_multicast_addr(self) -> Result<Self, CoffeeShopError> {
        let ip_addr = IpAddr::V4(self.multicast_host);

        if ip_addr.is_multicast() {
            Ok(self)
        } else {
            Err(CoffeeShopError::InvalidMulticastAddress(ip_addr))
        }
    }

    /// Builder pattern - change the Waiter address.
    pub fn with_host_addr(mut self, addr: SocketAddrV4) -> Self {
        self.port = addr.port();
        self.host = *addr.ip();

        self
    }

    /// Builder pattern - change the multicast address.
    pub fn with_multicast_addr(mut self, addr: SocketAddrV4) -> Result<Self, CoffeeShopError> {
        self.multicast_port = addr.port();
        self.multicast_host = *addr.ip();

        self.validate_multicast_addr()
    }

    /// Builder pattern - change the number of baristas to initiate.
    pub fn with_baristas(mut self, count: u16) -> Result<Self, CoffeeShopError> {
        if count == 0 {
            Err(CoffeeShopError::InvalidConfiguration {
                field: "baristas",
                message: format!("must be positive number, found {count}."),
            })
        } else {
            // Refuse to allow `0` baristas.
            self.baristas = count.max(1);
            Ok(self)
        }
    }

    /// Builder pattern - change the maximum number of tickets.
    pub fn with_max_tickets(mut self, count: usize) -> Result<Self, CoffeeShopError> {
        if count == 0 {
            Err(CoffeeShopError::InvalidConfiguration {
                field: "max_tickets",
                message: format!("must be positive number, found {count}."),
            })
        } else {
            self.max_tickets = count;
            Ok(self)
        }
    }

    /// Builder pattern - change the DynamoDB configuration.
    pub fn with_dynamodb_table(mut self, table: &str) -> Self {
        self.dynamodb_table = Some(table.to_owned());
        self
    }

    /// Builder pattern - change the DynamoDB partition key.
    pub fn with_dynamodb_partition_key(mut self, key: &str) -> Self {
        self.dynamodb_partition_key = key.to_owned();
        self
    }

    /// Builder pattern - change the result TTL.
    pub fn with_result_ttl(mut self, ttl: f32) -> Self {
        self.result_ttl = ttl;
        self
    }

    /// Builder pattern - change the SQS queue URL.
    pub fn with_sqs_queue(mut self, queue: String) -> Self {
        self.sqs_queue = Some(queue);
        self
    }
}

impl Config {
    /// Get the Multicast address in a packaged [`SocketAddr`] instance.
    pub fn multicast_addr(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(self.multicast_host), self.multicast_port)
    }

    /// Get the host address in a packaged [`SocketAddr`] instance.
    pub fn host_addr(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(self.host), self.port)
    }

    /// Get the DynamoDB TTL in [`tokio::time::Duration`] format.
    pub fn dynamodb_ttl(&self) -> tokio::time::Duration {
        tokio::time::Duration::from_secs_f32(self.result_ttl)
    }

    /// Get the maximum execution time in [`tokio::time::Duration`] format.
    pub fn max_execution_time(&self) -> Option<tokio::time::Duration> {
        self.max_execution_time
            .map(tokio::time::Duration::from_secs_f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! create_test {
        (
            $name:ident($builder:expr) -> $expected:expr
        ) => {
            #[test]
            fn $name() {
                let config = $builder;
                let expected = $expected;

                match (config, expected) {
                    (Ok(actual), Ok(expected)) => assert_eq!(actual, expected),
                    (Err(actual), Err(expected)) => {
                        assert_eq!(format!("{actual:?}"), format!("{expected:?}"));
                    }
                    (actual, expected) => {
                        panic!("Mismatched results; Expected: {expected:?} vs {actual:?}")
                    }
                }
            }
        };
    }

    create_test!(
        default(Ok::<_, CoffeeShopError>(Config::default())) -> Ok::<_, CoffeeShopError>(
            Config {
                ..Default::default()
            }
        )
    );
    create_test!(
        with_good_multicast_addr(
            Config::new().with_multicast_addr(
                SocketAddrV4::new(Ipv4Addr::new(224, 0, 0, 1), 1234)
            )
        ) -> Ok::<_, CoffeeShopError>(
            Config {
                multicast_host: Ipv4Addr::new(224,0,0,1),
                multicast_port: 1234,
                ..Default::default()
            }
        )
    );
    create_test!(
        with_bad_multicast_addr(
            Config::new().with_multicast_addr(
                SocketAddrV4::new(Ipv4Addr::new(192, 168, 0, 1), 4321)
            )
        ) -> Err(
            CoffeeShopError::InvalidMulticastAddress(
                IpAddr::V4(Ipv4Addr::new(192,168,0,1))
            )
        )
    );
    create_test!(
        with_bad_baristas(
            Config::new().with_baristas(0)
        ) -> Err(
            CoffeeShopError::InvalidConfiguration{
                field: "baristas",
                message: "must be positive number, found 0.".to_owned()
            }
        )
    );
    create_test!(
        with_good_baristas(
            Config::new().with_baristas(65535)
        ) -> Ok::<_, CoffeeShopError>(
            Config {
                baristas: 65535,
                ..Default::default()
            }
        )
    );
    create_test!(
        with_good_max_tickets(
            Config::new().with_max_tickets(2)
        ) -> Ok::<_, CoffeeShopError>(
            Config {
                max_tickets: 2,
                ..Default::default()
            }
        )
    );
    create_test!(
        with_bad_max_tickets(
            Config::new().with_max_tickets(0)
        ) -> Err(
            CoffeeShopError::InvalidConfiguration{
                field: "max_tickets",
                message: "must be positive number, found 0.".to_owned()
            }
        )
    );
}
