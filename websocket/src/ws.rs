use crate::handler::message::{on_message_send, ChatMessageIn};
use crate::handler::{ActiveMessage, ActiveMessageAction};
use crate::handler_finder::find_handler;
use crate::logging::debug;
use crate::{Client, Clients};
use futures::{FutureExt, StreamExt};
use mayhem_db::sea_orm::DatabaseConnection;
use serde_json::Error;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

pub async fn client_connection(ws: WebSocket, clients: Clients, db: Arc<DatabaseConnection>) {
    debug(format!("Establishing client connection: {:?}", ws).as_str());

    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            debug(format!("Error sending WebSocket message: {}", e).as_str());
        }
    }));

    let uuid = Uuid::new_v4().simple().to_string();

    let new_client = Client {
        client_id: uuid.clone(),
        sender: Some(client_sender),
    };

    clients.lock().await.insert(uuid.clone(), new_client);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                debug(
                    format!(
                        "Error receiving message for WebSocket (ID: {}): {}",
                        uuid.clone(),
                        e
                    )
                    .as_str(),
                );
                break;
            }
        };

        client_msg(&uuid, msg, &clients, db.clone()).await;
    }

    clients.lock().await.remove(&uuid);

    debug(format!("WebSocket disconnect: {}", uuid).as_str());
}

pub async fn client_msg(
    client_id: &str,
    msg: Message,
    clients: &Clients,
    db: Arc<DatabaseConnection>,
) {
    debug(format!("Received message from {}: {:?}", client_id, msg).as_str());

    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    let locked = clients.lock().await;

    match locked.get(client_id) {
        Some(client) => {
            if let Ok(res) = find_handler(message, &db, &client).await {
                if res.is_some() {
                    match res.unwrap() {
                        ActiveMessageAction::SendMessage => {
                            let json_parsed: Result<ActiveMessage<ChatMessageIn>, Error> =
                                serde_json::from_str(message);

                            let parsed = json_parsed.unwrap();

                            std::mem::drop(locked);

                            on_message_send(parsed.data, &db, &clients).await;

                            return;
                        }

                        _ => return,
                    };
                }

                return;
            }

            if message == "ping" || message == "ping\n" {
                if let Some(sender) = &client.sender {
                    debug(format!("Recieved ping from: {}; Sending pong.", client_id).as_str());

                    let _ = sender.send(Ok(Message::text("pong")));
                }

                return;
            }
        }

        None => return,
    };
}
