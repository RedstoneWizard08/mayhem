#![allow(clippy::needless_return)]

use sea_orm_migration::prelude::*;
use std::env;

#[tokio::main]
pub async fn main() {
    let key = "DATABASE_URL";

    if env::var(key).is_err() {
        env::set_var(key, "postgresql://mayhem:mayhem@127.0.0.1:5432/mayhem_dev");
    }

    cli::run_cli(migration::Migrator).await;
}
