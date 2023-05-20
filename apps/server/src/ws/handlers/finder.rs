use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::stream::SplitSink;
use serde::{Deserialize, Serialize};

use mayhem_db::sea_orm::DatabaseConnection;
use tokio::sync::{broadcast::Sender, Mutex};

use super::{
    channel::find_channel_handler, message::find_message_handler, server::find_server_handler,
    ActiveMessageAction,
};

use tracing::debug;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelAndServerIds {
    pub channel_id: i32,
    pub server_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAndServerIds {
    pub user_id: i32,
    pub server_id: i32,
}

pub async fn find_handler(
    message: &str,
    db: &DatabaseConnection,
    wtx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    sender: Sender<String>,
) -> Result<Option<ActiveMessageAction>, ()> {
    debug!("Trying to find a handler for the request.");

    // handler::message::on_*

    if let Ok(data) = find_message_handler(message, db, wtx.clone(), sender).await {
        return Ok(data);
    }

    // handler::channel::on_*

    if let Ok(data) = find_channel_handler(message, db, wtx.clone()).await {
        return Ok(data);
    }

    // handler::server::on_*

    if let Ok(data) = find_server_handler(message, db, wtx).await {
        return Ok(data);
    }

    debug!("Could not recognize a handler!");

    return Err(());
}
