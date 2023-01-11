use mayhem_db::{
    models::{message, Channel, ChatMessage},
    sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, ModelTrait},
};

use serde::{Deserialize, Serialize};
use warp::ws::Message;

use crate::{logging::warn::warn, Client, Clients};

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

pub async fn on_message_send(data: ChatMessageIn, db: &DatabaseConnection, clients: &Clients) {
    let msg = message::ActiveModel {
        channel_id: Set(data.channel),
        content: Set(data.content),
        timestamp: Set(data.timestamp),
        user_id: Set(data.sender),

        ..Default::default()
    };

    let message = msg.clone().insert(db).await.unwrap();

    let data_struct = ActiveMessage {
        action: ActiveMessageAction::RecieveMessage,
        data: message,
    };

    let data = serde_json::to_string(&data_struct).unwrap();

    let locked = clients.lock().await;

    for (client_id, client) in locked.iter() {
        if let Some(sender) = &client.sender {
            sender.send(Ok(Message::text(data.clone()))).unwrap();
        } else {
            warn(
                format!(
                    "Unable to send new message from user {} to client {}!",
                    msg.clone().user_id.into_value().unwrap(),
                    client_id
                )
                .as_str(),
            );

            return;
        }
    }
}

pub async fn on_get_all_messages(channel_id: i32, db: &DatabaseConnection, client: &Client) {
    let client_id = client.clone().client_id;
    let channel_res = Channel::find_by_id(channel_id.clone()).one(db).await;

    if let Err(err) = &channel_res {
        if let Some(sender) = &client.sender {
            sender.send(Ok(Message::text(err.to_string()))).unwrap();
        } else {
            warn(format!("Unable to send error log to client {}!", client_id).as_str());
        }

        return;
    }

    let channel_opt = channel_res.unwrap();

    if let Some(channel) = channel_opt {
        let messages_res = channel.find_related(ChatMessage).all(db).await;

        if let Err(err) = &messages_res {
            if let Some(sender) = &client.sender {
                sender.send(Ok(Message::text(err.to_string()))).unwrap();
            } else {
                warn(format!("Unable to send error log to client {}!", client_id).as_str());
            }

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

        if let Some(sender) = &client.sender {
            sender.send(Ok(Message::text(data))).unwrap();
        } else {
            warn(
                format!(
                    "Unable to send messages for channel {} to client {}!",
                    channel_id, client_id
                )
                .as_str(),
            );

            return;
        }
    } else {
        if let Some(sender) = &client.sender {
            sender
                .send(Ok(Message::text(
                    "Could not get the channel from the database!",
                )))
                .unwrap();
        } else {
            warn(format!("Unable to send error log to client {}!", client_id).as_str());
        }

        return;
    }
}
