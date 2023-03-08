use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use tokio::sync::Mutex;

use crate::{
    logging::{error, info},
    state::AppState,
};

use super::{message::handle_message, ChatRoom};

#[derive(Debug, PartialEq, Eq)]
pub enum MessageKind {
    PING,
    PONG,
    TEXT,
    BINARY,
    CLOSE,
}

pub fn process_message(msg: Message, who: SocketAddr) -> MessageKind {
    match msg {
        Message::Close(c) => {
            if let Some(cf) = c {
                info(
                    format!(
                        "{} sent close with code {} and reason `{}`",
                        who, cf.code, cf.reason
                    )
                    .as_str(),
                );
            } else {
                info(format!("{} somehow sent close message without CloseFrame", who).as_str());
            }

            return MessageKind::CLOSE;
        }

        Message::Ping(v) => {
            info(format!("{} sent ping with {:?}", who, v).as_str());

            return MessageKind::PING;
        }

        Message::Pong(v) => {
            info(format!("{} sent pong with {:?}", who, v).as_str());

            return MessageKind::PONG;
        }

        Message::Text(v) => {
            info(format!("{} sent text {}", who, v).as_str());

            return MessageKind::TEXT;
        }

        Message::Binary(v) => {
            info(format!("{} sent {} bytes: {:?}", who, v.len(), v).as_str());

            return MessageKind::BINARY;
        }
    };
}

pub async fn handle_socket(state: AppState, mut socket: WebSocket, who: SocketAddr) {
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        info(format!("Pinged {}...", who).as_str());
    } else {
        error(format!("Could not send ping {}!", who).as_str());
        return;
    }

    let room_id = 0;

    {
        let mut rooms = state.rooms.lock().await;

        if rooms.get(room_id).is_none() {
            rooms.push(ChatRoom::new());
        }
    }

    let rooms = state.rooms.lock().await.clone();
    let room = rooms.get(room_id).unwrap();

    let tx = room.sender.clone();
    let mut rx = room.sender.subscribe();

    let (wtx, mut wrx) = socket.split();

    let arc_wtx = Arc::new(Mutex::new(wtx));
    let arc_wtx_1 = arc_wtx.clone();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            println!("msg: {:?}", msg);

            arc_wtx_1
                .lock()
                .await
                .send(Message::Text(serde_json::to_string(&msg).unwrap()))
                .await
                .unwrap();
        }
    });

    let arc_wtx_2 = arc_wtx.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = wrx.next().await {
            let processed = process_message(msg.clone(), who);

            if processed == MessageKind::CLOSE {
                break;
            }

            handle_message(msg, state.clone(), arc_wtx_2.clone(), tx.clone()).await;
        }
    });

    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(_) => None,

                Err(a) => {
                    info(format!("Error sending messages: {:?}", a).as_str());
                    Some(())
                },
            };

            recv_task.abort();
        },

        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(_) => None,
                
                Err(b) => {
                    info(format!("Error receiving messages: {:?}", b).as_str());
                    Some(())
                },
            };

            send_task.abort();
        },
    }
}

pub async fn ws_handler(
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    return ws.on_upgrade(async move |socket| handle_socket(state, socket, addr).await);
}
