use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{stream::SplitSink, SinkExt};
use mayhem_db::{
    models::{user_server, EUserServer},
    sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter},
};
use tokio::sync::Mutex;

use crate::ws::handlers::{ActiveMessage, ActiveMessageAction};

pub async fn on_leave_server(
    user_id: i32,
    server_id: i32,
    db: &DatabaseConnection,
    wtx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
) {
    let user_server = EUserServer::find()
        .filter(user_server::Column::ServerId.eq(server_id))
        .filter(user_server::Column::UserId.eq(user_id))
        .one(db)
        .await
        .unwrap()
        .unwrap();

    user_server.delete(db).await.unwrap();

    let data_struct = ActiveMessage {
        action: ActiveMessageAction::LeaveServer,
        data: "success",
    };

    let data = serde_json::to_string(&data_struct).unwrap();

    wtx.lock().await.send(Message::Text(data)).await.unwrap();
}
