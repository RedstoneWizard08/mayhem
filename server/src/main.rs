#![allow(unused_must_use, unused_assignments)]
#![feature(proc_macro_hygiene, decl_macro, arc_unwrap_or_clone)]
#[macro_use]
extern crate rocket;

pub mod database;
pub mod errors;
pub mod logging;
pub mod middleware;
pub mod routes;
pub mod server;
pub mod state;
pub mod util;

use logging::custom;
use mayhem::database::prepare_connection;
use mayhem_db::Client;
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
        "Connecting to the database...",
    );

    let database_connection = prepare_connection();
    let client = Client::connect(database_connection).await;

    custom(
        custom::CustomType::WARN,
        "SETUP",
        "Creating request handler...",
    );

    let mut server = rocket::build();

    server = server.attach(DatabaseProvider::init());
    server = server.attach(LoggingMiddleware);
    server = server.manage(client.clone());

    server = server.mount("/", routes![handle_index, handle_login, handle_register]);
    server = server.attach(routes::server::stage());

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

    // let database = DatabaseProvider::fetch(&app).unwrap();
    // let client = database.get().await.unwrap();

    custom(custom::CustomType::WARN, "SETUP", "Running migrations...");

    client.run_migrations().await.unwrap();
    // run_migrations(&client).await;

    custom(
        custom::CustomType::INFO,
        "START",
        format!("Server listening on: {}:{}!", host, port).as_str(),
    );

    app.launch().await.unwrap();

    return Ok(());
}
