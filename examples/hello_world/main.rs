mod machine;
mod models;

use clap::Parser;
use coffeeshop::prelude::{Config, Shop};

/// Alias for the shop type, combining all the necessary types.
type HelloShop =
    Shop<models::HelloQuery, models::HelloPayload, models::HelloResult, machine::HelloMachine>;

/// The number of baristas to spawn in the shop.
const BARISTA_COUNT: usize = 3;

/// The most basic form of a coffee shop, using the test models.
#[tokio::main]
async fn main() -> Result<(), coffeeshop::CoffeeShopError> {
    let config = Config::parse();

    let shop = HelloShop::new(
        "hello-world".to_owned(),
        machine::HelloMachine::default(),
        config,
        None,
        BARISTA_COUNT,
    )
    .await?;

    shop.open(None, vec![].into_iter()).await
}
