#![allow(unused_must_use, unused_assignments, clippy::needless_return)]
#![feature(proc_macro_hygiene, decl_macro, arc_unwrap_or_clone, async_closure)]

#[cfg(not(debug_assertions))]
pub mod client;

pub mod config;
pub mod database;
pub mod errors;
pub mod logging;
pub mod middleware;
pub mod redis;
pub mod routes;
pub mod server;
pub mod state;
pub mod util;
pub mod ws;

use axum::{body::Body, middleware::from_fn, Router, Server};
use std::{error::Error, net::SocketAddr, sync::Arc};

#[cfg(not(debug_assertions))]
use client::run_client_node;

#[cfg(not(debug_assertions))]
use routes::client_handler;

#[cfg(debug_assertions)]
use routes::handle_error;

#[cfg(debug_assertions)]
use logging::warn;

use crate::config::get_config;
use database::prepare_connection;
use logging::info;
use mayhem_db::Client;
use middleware::logger::logging_middleware;
use state::AppState;
use util::parse_ip;

#[cfg(not(debug_assertions))]
async fn _run_client() {
    info("Starting client...");

    tokio::spawn(async {
        run_client_node().await;
    });
}

#[cfg(debug_assertions)]
async fn _run_client() {
    warn("Not starting client, is a debug build.");
}

#[cfg(not(debug_assertions))]
fn _register_client_handler(router: Router<AppState, Body>) -> Router<AppState, Body> {
    return router.fallback(client_handler);
}

#[cfg(debug_assertions)]
fn _register_client_handler(router: Router<AppState, Body>) -> Router<AppState, Body> {
    return router.fallback(handle_error);
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let config = get_config().await;

    info("Config parsed!");

    let database_connection = prepare_connection(config.clone());
    let client = Arc::new(Client::connect(database_connection).await);

    info("Connected to the database!");

    let state = AppState::new(client.clone(), config.clone());

    let router = Router::new();

    let router = routes::register(router);
    let router = ws::register(router);
    let router = _register_client_handler(router);
    let router = router.layer(from_fn(logging_middleware));

    let router = router.with_state(state);

    client.run_migrations().await.unwrap();

    info("Migrations succeeded!");

    let ip = parse_ip(config.clone().host);
    let address = SocketAddr::from((ip, config.clone().port));
    let server = Server::bind(&address);
    let service = router.into_make_service_with_connect_info::<SocketAddr>();
    let app = server.serve(service);

    _run_client().await;

    info(format!("Listening on {}", address).as_str());

    app.await.unwrap();

    return Ok(());
}
