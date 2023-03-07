#![allow(unused_must_use, unused_assignments)]
#![feature(proc_macro_hygiene, decl_macro, arc_unwrap_or_clone, async_closure)]

pub mod config;
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
    config::app::get_config, database::prepare_connection, logging::info,
    middleware::logger::logging_middleware, routes::handle_error, util::parse_ip,
};

use mayhem_db::Client;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let config = get_config().await;

    info("Config parsed!");

    let database_connection = prepare_connection(config.clone());
    let client = Arc::new(Client::connect(database_connection).await);

    info("Connected to the database!");

    let router = Router::new();

    let router = routes::register(router);
    let router = router.fallback(handle_error);
    let router = router.layer(from_fn(logging_middleware));

    let router = router.with_state(client.clone());

    client.run_migrations().await.unwrap();

    info("Migrations succeeded!");

    let ip = parse_ip(config.clone().host);
    let address = SocketAddr::from((ip, config.clone().port));
    let server = Server::bind(&address);
    let app = server.serve(router.into_make_service());

    info(format!("Listening on {}", address).as_str());

    app.await.unwrap();

    return Ok(());
}
