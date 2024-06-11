use mayhem_db::{
    models::message,
    sea_orm::{ActiveModelTrait, DatabaseConnection, Set},
};
use tokio::sync::broadcast::Sender;
use tracing::debug;

use crate::ws::handlers::{ActiveMessage, ActiveMessageAction};

use super::ChatMessageIn;

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
