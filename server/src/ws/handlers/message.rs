use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{stream::SplitSink, SinkExt};
use mayhem_db::{
    models::{message, EChannel, EChatMessage},
    sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, ModelTrait},
};

use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, broadcast::Sender};
use tracing::debug;

use super::{ActiveMessage, ActiveMessageAction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessageIn {
    pub content: String,
    pub timestamp: String,
    pub sender: i32,
    pub channel: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessagesData {
    pub channel_id: i32,
    pub messages: Vec<message::Model>,
}

pub async fn on_message_send(data: ChatMessageIn, db: &DatabaseConnection, sender: Sender<String>) {
    debug!("Creating active model for the new message...");

    let msg = message::ActiveModel {
        channel_id: Set(data.channel),
        content: Set(data.content),
        timestamp: Set(data.timestamp),
        user_id: Set(data.sender),

        ..Default::default()
    };

    debug!("Inserting the message.");

    let message = msg.clone().insert(db).await.unwrap();

    debug!("Creating the data struct to send to the client.");

    let data_struct = ActiveMessage {
        action: ActiveMessageAction::RecieveMessage,
        data: message,
    };

    debug!("Serializing the data.");

    let data = serde_json::to_string(&data_struct).unwrap();

    debug!("Sending to clients.");

    sender.send(data);
}

pub async fn on_get_all_messages(
    channel_id: i32,
    db: &DatabaseConnection,
    wtx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
) {
    let channel_res = EChannel::find_by_id(channel_id.clone()).one(db).await;

    if let Err(err) = &channel_res {
        wtx.lock().await.send(Message::Text(err.to_string())).await.unwrap();

        return;
    }

    let channel_opt = channel_res.unwrap();

    if let Some(channel) = channel_opt {
        let messages_res = channel.find_related(EChatMessage).all(db).await;

        if let Err(err) = &messages_res {
            wtx.lock().await.send(Message::Text(err.to_string())).await.unwrap();

            return;
        }

        let messages = messages_res.unwrap();

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
        wtx.lock().await.send(Message::Text(
            "Could not get the channel from the database!".to_string(),
        ))
        .await
        .unwrap();
    }
}
