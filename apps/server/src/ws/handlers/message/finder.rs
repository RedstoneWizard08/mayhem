use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::stream::SplitSink;
use mayhem_db::sea_orm::DatabaseConnection;
use serde_json::Error;
use tokio::sync::{broadcast::Sender, Mutex};
use tracing::debug;

use crate::ws::handlers::{
    message::{on_get_all_messages, on_message_send},
    ActiveMessage, ActiveMessageAction,
};

use super::ChatMessageIn;

pub async fn find_message_handler(
    message: &str,
    db: &DatabaseConnection,
    wtx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    sender: Sender<String>,
) -> Result<Option<ActiveMessageAction>, ()> {
    let json_parsed: Result<ActiveMessage<ChatMessageIn>, Error> = serde_json::from_str(message);

    if let Ok(parsed) = json_parsed {
        debug!("Parse ok: on_message_send");

        if parsed.action == ActiveMessageAction::SendMessage {
            debug!("Recognized on_message_send...");
            on_message_send(parsed.data, db, sender).await;
            return Ok(Some(ActiveMessageAction::SendMessage));
        }
    }

    let json_parsed: Result<ActiveMessage<i32>, Error> = serde_json::from_str(message);

    if let Ok(parsed) = json_parsed {
        debug!("Parse ok: on_get_all_messages");

        if parsed.action == ActiveMessageAction::GetMessagesForChannel {
            debug!("Recognized on_get_all_messages...");
            on_get_all_messages(parsed.data, db, wtx).await;
            return Ok(None);
        }
    }

    return Err(());
}
