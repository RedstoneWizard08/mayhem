use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::stream::SplitSink;
use mayhem_db::sea_orm::DatabaseConnection;
use serde_json::Error;
use tokio::sync::Mutex;
use tracing::debug;

use crate::ws::handlers::{finder::ChannelAndServerIds, ActiveMessage, ActiveMessageAction};

use super::{on_create_channel, on_get_channel, on_get_channels, ChannelCreateData};

pub async fn find_channel_handler(
    message: &str,
    db: &DatabaseConnection,
    wtx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
) -> Result<Option<ActiveMessageAction>, ()> {
    let json_parsed: Result<ActiveMessage<ChannelCreateData>, Error> =
        serde_json::from_str(message);

    if let Ok(parsed) = json_parsed {
        debug!("Parse ok: on_create_channel");

        if parsed.action == ActiveMessageAction::CreateChannel {
            debug!("Recognized on_create_channel...");
            on_create_channel(parsed.data, db, wtx).await;
            return Ok(None);
        }
    }

    let json_parsed: Result<ActiveMessage<ChannelAndServerIds>, Error> =
        serde_json::from_str(message);

    if let Ok(parsed) = json_parsed {
        debug!("Parse ok: on_get_channel");

        if parsed.action == ActiveMessageAction::GetChannelInfo {
            debug!("Recognized on_get_channel...");
            on_get_channel(parsed.data.server_id, parsed.data.channel_id, db, wtx).await;
            return Ok(None);
        }
    }

    let json_parsed: Result<ActiveMessage<i32>, Error> = serde_json::from_str(message);

    if let Ok(parsed) = json_parsed {
        debug!("Parse ok: on_get_channels");

        if parsed.action == ActiveMessageAction::GetChannelsInServer {
            debug!("Recognized on_get_channels...");
            on_get_channels(parsed.data, db, wtx).await;
            return Ok(None);
        }
    }

    return Err(());
}
