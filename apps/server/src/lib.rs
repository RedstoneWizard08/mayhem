pub mod config;
pub mod database;
pub mod glue;
pub mod log;
pub mod redis;
pub mod router;
pub mod routes;
pub mod server;
pub mod state;
pub mod ws;

use crate::config::get_config;
use anyhow::Result;
use axum::serve;
use database::connect;
use glue::{abort::register_exit_handler, make_glue};
use log::init_logger;
use mayhem_db::migrate;
use router::RouterBuilder;
use state::AppState;
use std::{error::Error, net::SocketAddr};
use tokio::{join, net::TcpListener};
use tracing::{info, level_filters::LevelFilter};

pub async fn start() -> Result<()> {
    init_logger("logs/server.log", LevelFilter::current());

    info!("Registering CTRL+C handler...");

    register_exit_handler()?;

    info!("Registered!");
    info!("Parsing config...");

    let config = get_config().await;

    info!("Config parsed!");
    info!("Connecting to the database...");

    let pool = connect(config)?;

    info!("Connected!");
    info!("Running migrations...");

    migrate(pool).await?;

    info!("Migrations succeeded!");
    info!("Creating glue...");

    let glue = make_glue()?;

    info!("Created!");
    info!("Creating router...");

    let state = AppState::new(pool, config.clone());

    let router = RouterBuilder::new()
        .glue(glue.clone())
        .routes()
        .log()
        .build(state)
        .into_make_service_with_connect_info::<SocketAddr>();

    info!("Created!");
    info!("Initializing server...");

    let ip: IpAddr = config.host.parse()?;
    let addr = SocketAddr::from((ip, config.port));
    let listener = TcpListener::bind(&addr).await?;

    info!("Initialized!");
    info!("Listening on {}:{}!", cli.host, cli.port);

    let server = tokio::spawn(async move { serve(listener, router).await });

    if is_debug() {
        info!("Starting client...");

        let client = glue.spawn().await;
        let (a, b) = join!(client, server);

        a?;
        b??;

        return Ok(());
    }

    server.await??;

    return Ok(());
}
