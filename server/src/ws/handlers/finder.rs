use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::stream::SplitSink;
use serde::{Deserialize, Serialize};
use serde_json::Error;

use mayhem_db::sea_orm::DatabaseConnection;
use tokio::sync::{broadcast::Sender, Mutex};

use crate::ws::handlers::server::on_get_servers;

use super::{
    channel::{on_create_channel, on_get_channel, on_get_channels, ChannelCreateData},
    message::{on_get_all_messages, on_message_send, ChatMessageIn},
    server::{on_create_server, on_get_server, on_join_server, on_leave_server, ServerCreateData},
    ActiveMessage, ActiveMessageAction,
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

    let json_parsed: Result<ActiveMessage<ChatMessageIn>, Error> = serde_json::from_str(message);

    if let Ok(parsed) = json_parsed {
        debug!("Parse ok: on_message_send");

        if parsed.action == ActiveMessageAction::SendMessage {
            debug!("Recognized on_message_send...");
            on_message_send(parsed.data, &db, sender).await;
            return Ok(Some(ActiveMessageAction::SendMessage));
        }
    }

    let json_parsed: Result<ActiveMessage<i32>, Error> = serde_json::from_str(message);

    if let Ok(parsed) = json_parsed {
        debug!("Parse ok: on_get_all_messages");

        if parsed.action == ActiveMessageAction::GetMessagesForChannel {
            debug!("Recognized on_get_all_messages...");
            on_get_all_messages(parsed.data, &db, wtx).await;
            return Ok(None);
        }
    }

    // handler::channel::on_*

    let json_parsed: Result<ActiveMessage<ChannelCreateData>, Error> =
        serde_json::from_str(message);

    if let Ok(parsed) = json_parsed {
        debug!("Parse ok: on_create_channel");

        if parsed.action == ActiveMessageAction::CreateChannel {
            debug!("Recognized on_create_channel...");
            on_create_channel(parsed.data, &db, wtx).await;
            return Ok(None);
        }
    }

    let json_parsed: Result<ActiveMessage<ChannelAndServerIds>, Error> =
        serde_json::from_str(message);

    if let Ok(parsed) = json_parsed {
        debug!("Parse ok: on_get_channel");

        if parsed.action == ActiveMessageAction::GetChannelInfo {
            debug!("Recognized on_get_channel...");
            on_get_channel(parsed.data.server_id, parsed.data.channel_id, &db, wtx).await;
            return Ok(None);
        }
    }

    let json_parsed: Result<ActiveMessage<i32>, Error> = serde_json::from_str(message);

    if let Ok(parsed) = json_parsed {
        debug!("Parse ok: on_get_channels");

        if parsed.action == ActiveMessageAction::GetChannelsInServer {
            debug!("Recognized on_get_channels...");
            on_get_channels(parsed.data, &db, wtx).await;
            return Ok(None);
        }
    }

    // handler::server::on_*

    let json_parsed: Result<ActiveMessage<ServerCreateData>, Error> = serde_json::from_str(message);

    if let Ok(parsed) = json_parsed {
        debug!("Parse ok: on_create_server");

        if parsed.action == ActiveMessageAction::CreateServer {
            debug!("Recognized on_create_server...");
            on_create_server(parsed.data, &db, wtx).await;
            return Ok(None);
        }
    }

    let json_parsed: Result<ActiveMessage<i32>, Error> = serde_json::from_str(message);

    if let Ok(parsed) = json_parsed {
        debug!("Parse ok: on_get_server");

        if parsed.action == ActiveMessageAction::GetServerInfo {
            debug!("Recognized on_get_server...");
            on_get_server(parsed.data, &db, wtx).await;
            return Ok(None);
        }

        if parsed.action == ActiveMessageAction::GetServersForUser {
            debug!("Recognized on_get_servers...");
            on_get_servers(parsed.data, &db, wtx).await;
            return Ok(None);
        }
    }

    let json_parsed: Result<ActiveMessage<UserAndServerIds>, Error> = serde_json::from_str(message);

    if let Ok(parsed) = json_parsed {
        debug!("Parse ok: on_join_server | on_leave_server");

        if parsed.action == ActiveMessageAction::JoinServer {
            debug!("Recognized on_join_server...");
            on_join_server(parsed.data.user_id, parsed.data.server_id, &db, wtx).await;
            return Ok(None);
        }

        if parsed.action == ActiveMessageAction::LeaveServer {
            debug!("Recognized on_leave_server...");
            on_leave_server(parsed.data.user_id, parsed.data.server_id, &db, wtx).await;
            return Ok(None);
        }
    }

    debug!("Could not recognize a handler!");

    return Err(());
}
