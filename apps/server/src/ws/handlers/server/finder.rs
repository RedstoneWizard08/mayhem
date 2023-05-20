use std::sync::Arc;

use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use futures::stream::SplitSink;
use mayhem_db::sea_orm::DatabaseConnection;
use serde_json::Error;
use tokio::sync::Mutex;
use tracing::debug;

use crate::ws::handlers::finder::UserAndServerIds;
use crate::ws::handlers::{ActiveMessage, ActiveMessageAction};

use super::on_create_server;
use super::on_delete_server;
use super::on_get_server;
use super::on_get_servers;
use super::on_join_server;
use super::on_leave_server;
use super::ServerCreateData;

pub async fn find_server_handler(
    message: &str,
    db: &DatabaseConnection,
    wtx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
) -> Result<Option<ActiveMessageAction>, ()> {
    let json_parsed: Result<ActiveMessage<ServerCreateData>, Error> = serde_json::from_str(message);

    if let Ok(parsed) = json_parsed {
        debug!("Parse ok: on_create_server");

        if parsed.action == ActiveMessageAction::CreateServer {
            debug!("Recognized on_create_server...");
            on_create_server(parsed.data, db, wtx).await;
            return Ok(None);
        }
    }

    let json_parsed: Result<ActiveMessage<i32>, Error> = serde_json::from_str(message);

    if let Ok(parsed) = json_parsed {
        debug!("Parse ok: on_get_server | on_get_servers | on_delete_servers");

        if parsed.action == ActiveMessageAction::GetServerInfo {
            debug!("Recognized on_get_server...");
            on_get_server(parsed.data, db, wtx).await;
            return Ok(None);
        }

        if parsed.action == ActiveMessageAction::GetServersForUser {
            debug!("Recognized on_get_servers...");
            on_get_servers(parsed.data, db, wtx).await;
            return Ok(None);
        }

        if parsed.action == ActiveMessageAction::DeleteServer {
            debug!("Recognized on_delete_servers...");
            on_delete_server(parsed.data, db, wtx).await;
            return Ok(None);
        }
    }

    let json_parsed: Result<ActiveMessage<UserAndServerIds>, Error> = serde_json::from_str(message);

    if let Ok(parsed) = json_parsed {
        debug!("Parse ok: on_join_server | on_leave_server");

        if parsed.action == ActiveMessageAction::JoinServer {
            debug!("Recognized on_join_server...");
            on_join_server(parsed.data.user_id, parsed.data.server_id, db, wtx).await;
            return Ok(None);
        }

        if parsed.action == ActiveMessageAction::LeaveServer {
            debug!("Recognized on_leave_server...");
            on_leave_server(parsed.data.user_id, parsed.data.server_id, db, wtx).await;
            return Ok(None);
        }
    }

    return Err(());
}
