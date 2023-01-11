use crate::{ws, Clients, Result};
use mayhem_db::sea_orm::DatabaseConnection;
use std::sync::Arc;
use warp::Reply;

pub async fn ws_handler(
    ws: warp::ws::Ws,
    clients: Clients,
    db: Arc<DatabaseConnection>,
) -> Result<impl Reply> {
    println!("ws_handler");

    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, clients, db)))
}
