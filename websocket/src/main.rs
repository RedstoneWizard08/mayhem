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

use logging::info;
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

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

pub type Clients = Arc<Mutex<HashMap<String, Client>>>;
pub type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
pub async fn main() {
    let database_connection = prepare_connection();
    let client = DbClient::connect(database_connection).await;

    info("Connected to the database!");

    client.run_migrations().await.unwrap();

    info("Migrations succeeded!");

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and(with_database(client.client.clone()))
        .and_then(handlers::ws_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());
    info("Starting server on 127.0.0.1:4002");
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
