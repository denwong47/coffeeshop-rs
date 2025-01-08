//! A simple example of how to setup and run a [`Shop`].

mod machine;
mod models;

use clap::Parser;
use coffeeshop::prelude::{Config, Shop};

#[cfg(doc)]
use coffeeshop::prelude::*;

/// Alias for the shop type, combining all the necessary types.
type HelloShop =
    Shop<models::HelloQuery, models::HelloPayload, models::HelloResult, machine::HelloMachine>;

/// The most basic form of a coffee shop, using the test models.
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), coffeeshop::CoffeeShopError> {
    let config = Config::parse();

    let shop = HelloShop::new(
        "hello-world".to_owned(),
        machine::HelloMachine::default(),
        config,
        None,
    )
    .await?;

    shop.open(None, vec![].into_iter()).await
}
