#![allow(unused_must_use, unused_assignments)]
#![feature(proc_macro_hygiene, decl_macro, arc_unwrap_or_clone, async_closure)]

#[macro_use]
extern crate tracing;

pub mod database;
pub mod errors;
pub mod logging;
pub mod middleware;
pub mod routes;
pub mod server;
pub mod util;

use std::{error::Error, net::SocketAddr, sync::Arc};

use axum::{middleware::from_fn, Router, Server};
use mayhem::{
    database::prepare_connection, middleware::logger::logging_middleware, routes::handle_error,
};
use mayhem_db::Client;

pub async fn get_root() -> &'static str {
    return "Hello, world!";
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let database_connection = prepare_connection();
    let client = Arc::new(Client::connect(database_connection).await);

    info!("Connected to the database!");

    let router = Router::new();

    let router = routes::register(router);
    let router = router.fallback(handle_error);
    let router = router.layer(from_fn(logging_middleware));

    let router = router.with_state(client.clone());

    client.run_migrations().await.unwrap();

    info!("Migrations succeeded!");

    let address = SocketAddr::from(([0, 0, 0, 0], 4001));
    let server = Server::bind(&address);
    let app = server.serve(router.into_make_service());

    info!("Listening on {}", address);

    app.await.unwrap();

    return Ok(());
}
