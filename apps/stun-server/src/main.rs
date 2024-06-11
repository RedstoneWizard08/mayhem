use tokio::main;

#[macro_use]
extern crate lazy_static;

pub mod broadcaster;
pub mod signal;
pub mod turn;

use anyhow::Result;
use turn::start_turn_server;

#[main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    start_turn_server().await?;

    return Ok(());
}
