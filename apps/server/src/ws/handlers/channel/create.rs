use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{stream::SplitSink, SinkExt};
use mayhem_db::{
    models::server::channel,
    sea_orm::{ActiveModelTrait, DatabaseConnection, Set},
};
use tokio::sync::Mutex;

use crate::ws::handlers::{ActiveMessage, ActiveMessageAction};

use super::ChannelCreateData;

pub async fn on_create_channel(
    creation_data: ChannelCreateData,
    db: &DatabaseConnection,
    wtx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
) {
    let channel_data = channel::ActiveModel {
        name: Set(creation_data.name),
        server_id: Set(creation_data.server_id),
        channel_type: Set(creation_data.channel_type),

        ..Default::default()
    };

    let channel = channel_data.clone().insert(db).await.unwrap();

    let data_struct = ActiveMessage {
        action: ActiveMessageAction::CreateChannel,
        data: channel.clone(),
    };

    let data = serde_json::to_string(&data_struct).unwrap();

    wtx.lock().await.send(Message::Text(data)).await.unwrap();
}
