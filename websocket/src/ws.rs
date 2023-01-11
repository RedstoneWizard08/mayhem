use crate::handler_finder::find_handler;
use crate::{Client, Clients};
use futures::{FutureExt, StreamExt};
use mayhem_db::sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

pub async fn client_connection(ws: WebSocket, clients: Clients, db: Arc<DatabaseConnection>) {
    println!("establishing client connection... {:?}", ws);

    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
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
                println!("error receiving message for id {}): {}", uuid.clone(), e);
                break;
            }
        };

        client_msg(&uuid, msg, &clients, db.clone()).await;
    }

    clients.lock().await.remove(&uuid);
    println!("{} disconnected", uuid);
}

pub async fn client_msg(
    client_id: &str,
    msg: Message,
    clients: &Clients,
    db: Arc<DatabaseConnection>,
) {
    println!("received message from {}: {:?}", client_id, msg);

    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    let locked = clients.lock().await;
    match locked.get(client_id) {
        Some(client) => {
            if let Ok(_) = find_handler(message, &db, &clients, &client).await {
                return;
            }

            if message == "ping" || message == "ping\n" {
                if let Some(sender) = &client.sender {
                    println!("sending pong");
                    let _ = sender.send(Ok(Message::text("pong")));
                }
                return;
            }
        }
        None => return,
    };
}
