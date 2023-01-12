pub mod handler;
pub mod handler_finder;
pub mod handlers;
pub mod logging;
pub mod ws;

use mayhem_db::{
    client::connector::{
        Authentication, ConnectionOptions, DatabaseProtocol, PasswordAuthentication,
    },
    sea_orm::DatabaseConnection,
    Client as DbClient,
};

use logging::{debug, info};
use sqlx::{postgres::PgPoolOptions, Row};
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{ws::Message, Filter, Rejection};

pub fn prepare_connection() -> ConnectionOptions {
    return ConnectionOptions {
        protocol: DatabaseProtocol::PostgreSQL,
        auth: Authentication::Password(PasswordAuthentication {
            user: "mayhem".to_string(),
            pass: "mayhem".to_string(),
        }),
        host: "127.0.0.1".to_string(),
        port: 5432,
        database: "mayhem_dev".to_string(),
    };
}

pub fn migration_connection_string() -> String {
    return "postgresql://mayhem:mayhem@127.0.0.1:5432/postgres".to_string();
}

pub fn migration_connection_string_mayhem_dev() -> String {
    return "postgresql://mayhem:mayhem@127.0.0.1:5432/mayhem_dev".to_string();
}

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

pub type Clients = Arc<Mutex<HashMap<String, Client>>>;
pub type Result<T> = std::result::Result<T, Rejection>;

pub async fn do_migrate() {
    debug(
        format!(
            "Connecting to PostgreSQL via connection URI: {}",
            &migration_connection_string()
        )
        .as_str(),
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&migration_connection_string())
        .await
        .unwrap();

    debug(
        format!(
            "Connected to PostgreSQL via connection URI: {}",
            &migration_connection_string()
        )
        .as_str(),
    );

    info("[MIGRATE > CREATION CHECK] Connected to the database!");

    debug("Running query: \"SELECT datname FROM pg_database;\"");

    let dbs = sqlx::query("SELECT datname FROM pg_database;")
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut is_found = false;

    debug("Checking for \"mayhem_dev\" database...");

    for db in dbs {
        let db_name: String = db.get("datname");

        debug(
            format!(
                "[MATCH CHECK] \"{}\" == \"mayhem_dev\" ? {}",
                db_name,
                db_name == "mayhem_dev"
            )
            .as_str(),
        );

        if db_name == "mayhem_dev" {
            debug("[MATCH CHECK] Database exists. Skipping creation.");
            is_found = true;
            break;
        }
    }

    if !is_found {
        debug("Creating \"mayhem_dev\" database...");
        debug("Running query: \"CREATE DATABASE mayhem_dev WITH OWNER mayhem;\"");

        sqlx::query("CREATE DATABASE mayhem_dev WITH OWNER mayhem;")
            .execute(&pool)
            .await
            .unwrap();

        debug(
            format!(
                "Connecting to PostgreSQL via connection URI: {}",
                &migration_connection_string_mayhem_dev()
            )
            .as_str(),
        );

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&migration_connection_string_mayhem_dev())
            .await
            .unwrap();

        debug(
            format!(
                "Connected to PostgreSQL via connection URI: {}",
                &migration_connection_string_mayhem_dev()
            )
            .as_str(),
        );

        info("[MIGRATE > PERMISSIONS] Connected to the database!");

        debug("Running query: \"GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO mayhem;\"");
        sqlx::query("GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO mayhem;")
            .execute(&pool)
            .await
            .unwrap();

        debug(
            "Running query: \"GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO mayhem;\"",
        );
        sqlx::query("GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO mayhem;")
            .execute(&pool)
            .await
            .unwrap();
    }

    debug("Preparing migration client...");

    let database_connection = prepare_connection();

    debug(
        format!(
            "Connecting to PostgreSQL via connection URI: {}",
            &database_connection.get_connection_string()
        )
        .as_str(),
    );

    let client = DbClient::connect(database_connection.clone()).await;

    info("[MIGRATE] Connected to the database!");

    debug(
        format!(
            "Connected to PostgreSQL via connection URI: {}",
            database_connection.get_connection_string()
        )
        .as_str(),
    );
    debug("Running migrations...");

    client.run_migrations().await.unwrap();

    info("Migrations succeeded!");
    debug("Migrations completed.");
}

#[tokio::main]
pub async fn main() {
    debug("Running database check and migrations...");

    do_migrate().await;

    debug("Preparing server connection...");

    let database_connection = prepare_connection();

    debug(
        format!(
            "Connecting to PostgreSQL via connection URI: {}",
            &database_connection.get_connection_string()
        )
        .as_str(),
    );

    let client = DbClient::connect(database_connection.clone()).await;

    info("Connected to the database!");
    debug(
        format!(
            "Connected to PostgreSQL via connection URI: {}",
            &database_connection.get_connection_string()
        )
        .as_str(),
    );

    debug("Setting up clients...");

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    debug("Setting up routes...");

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and(with_database(client.client.clone()))
        .and_then(handlers::ws_handler);

    debug("Finalizing route configuration...");

    let routes = ws_route.with(warp::cors().allow_any_origin());

    info("Starting server on 127.0.0.1:4002");
    debug("Starting listening on 127.0.0.1:4002");

    warp::serve(routes).run(([127, 0, 0, 1], 4002)).await;
}

pub fn with_clients(
    clients: Clients,
) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

pub fn with_database(
    conn: Arc<DatabaseConnection>,
) -> impl Filter<Extract = (Arc<DatabaseConnection>,), Error = Infallible> + Clone {
    warp::any().map(move || conn.clone())
}
