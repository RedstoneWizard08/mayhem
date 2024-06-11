use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{stream::SplitSink, SinkExt};
use mayhem_db::{
    models::server::server,
    sea_orm::{ActiveModelTrait, DatabaseConnection, Set},
};
use tokio::sync::Mutex;

use crate::ws::handlers::{ActiveMessage, ActiveMessageAction};

use super::ServerCreateData;

pub async fn on_create_server(
    creation_data: ServerCreateData,
    db: &DatabaseConnection,
    wtx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
) {
    let server_data = server::ActiveModel {
        name: Set(creation_data.name),
        ..Default::default()
    };

    let server = server_data.clone().insert(db).await.unwrap();

    let data_struct = ActiveMessage {
        action: ActiveMessageAction::CreateServer,
        data: server.clone(),
    };

    let data = serde_json::to_string(&data_struct).unwrap();

    wtx.lock().await.send(Message::Text(data)).await.unwrap();
}
