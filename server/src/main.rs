#![allow(unused_must_use, unused_assignments)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

pub mod database;
pub mod errors;
pub mod logging;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod state;
pub mod util;

use database::migrate::run_migrations;
use logging::custom;
use middleware::logger::LoggingMiddleware;
use rocket::{Config as RocketConfig, Error};
use rocket_db_pools::Database;
use routes::{handle_index, handle_login, handle_register};
use state::DatabaseProvider;

#[main]
pub async fn main() -> Result<(), Error> {
    custom(
        custom::CustomType::WARN,
        "SETUP",
        "Creating request handler...",
    );

    let mut server = rocket::build();

    server = server.attach(DatabaseProvider::init());
    server = server.attach(LoggingMiddleware);

    server = server.mount("/", routes![handle_index, handle_login, handle_register]);

    let figment = server.figment();
    let config: RocketConfig = figment.extract().expect("config");
    let host = config.address.to_string();
    let port = config.port;

    custom(
        custom::CustomType::WARN,
        "SETUP",
        "Finalizing and verifying configuration...",
    );

    let app = server.ignite().await.unwrap();
    let database = DatabaseProvider::fetch(&app).unwrap();
    let client = database.get().await.unwrap();

    custom(custom::CustomType::WARN, "SETUP", "Running migrations...");

    run_migrations(&client).await;

    custom(
        custom::CustomType::INFO,
        "START",
        format!("Server listening on: {}:{}!", host, port).as_str(),
    );

    app.launch().await.unwrap();

    return Ok(());
}
