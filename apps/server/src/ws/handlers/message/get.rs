use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{stream::SplitSink, SinkExt};
use mayhem_db::{
    models::{EChannel, EChatMessage},
    sea_orm::{DatabaseConnection, EntityTrait, ModelTrait},
};
use tokio::sync::Mutex;

use crate::ws::handlers::{ActiveMessage, ActiveMessageAction};

use super::MessagesData;

pub async fn on_get_all_messages(
    channel_id: i32,
    db: &DatabaseConnection,
    wtx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
) {
    let channel_res = EChannel::find_by_id(channel_id).one(db).await;

    if let Err(err) = &channel_res {
        wtx.lock()
            .await
            .send(Message::Text(err.to_string()))
            .await
            .unwrap();

        return;
    }

    let channel_opt = channel_res.unwrap();

    if let Some(channel) = channel_opt {
        let messages_res = channel.find_related(EChatMessage).all(db).await;

        if let Err(err) = &messages_res {
            wtx.lock()
                .await
                .send(Message::Text(err.to_string()))
                .await
                .unwrap();

            return;
        }

        let messages = messages_res.unwrap();
        let messages = messages.iter().rev().take(20).rev().cloned().collect();

        let data_struct = ActiveMessage {
            action: ActiveMessageAction::GetMessagesForChannel,
            data: MessagesData {
                channel_id,
                messages,
            },
        };

        let data = serde_json::to_string(&data_struct).unwrap();

        wtx.lock().await.send(Message::Text(data)).await.unwrap();
    } else {
        wtx.lock()
            .await
            .send(Message::Text(
                "Could not get the channel from the database!".to_string(),
            ))
            .await
            .unwrap();
    }
}
