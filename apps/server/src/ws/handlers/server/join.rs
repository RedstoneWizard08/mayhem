use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{stream::SplitSink, SinkExt};
use mayhem_db::{
    models::user_server,
    sea_orm::{ActiveModelTrait, DatabaseConnection, Set},
};
use tokio::sync::Mutex;

use crate::ws::handlers::{ActiveMessage, ActiveMessageAction};

pub async fn on_join_server(
    user_id: i32,
    server_id: i32,
    db: &DatabaseConnection,
    wtx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
) {
    let user_server_data = user_server::ActiveModel {
        server_id: Set(server_id),
        user_id: Set(user_id),
    };

    let user_server = user_server_data.clone().insert(db).await.unwrap();

    let data_struct = ActiveMessage {
        action: ActiveMessageAction::JoinServer,
        data: user_server,
    };

    let data = serde_json::to_string(&data_struct).unwrap();

    wtx.lock().await.send(Message::Text(data)).await.unwrap();
}
