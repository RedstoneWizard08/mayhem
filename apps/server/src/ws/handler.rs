use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};

use super::{message::handle_message, ChatRoom};
use crate::state::AppState;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, PartialEq, Eq)]
pub enum MessageKind {
    PING,
    PONG,
    TEXT,
    BINARY,
    CLOSE,
}

pub fn process_message(msg: Message) -> MessageKind {
    match msg {
        Message::Close(_) => {
            return MessageKind::CLOSE;
        }

        Message::Ping(_) => {
            return MessageKind::PING;
        }

        Message::Pong(_) => {
            return MessageKind::PONG;
        }

        Message::Text(_) => {
            return MessageKind::TEXT;
        }

        Message::Binary(_) => {
            return MessageKind::BINARY;
        }
    };
}

pub async fn handle_socket(state: AppState, mut socket: WebSocket) {
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_err() {
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
            let processed = process_message(msg.clone());

            if processed == MessageKind::CLOSE {
                break;
            }

            handle_message(msg, state.clone(), arc_wtx_2.clone(), tx.clone()).await;
        }
    });

    tokio::select! {
        _rv_a = (&mut send_task) => {
            recv_task.abort();
        },

        _rv_b = (&mut recv_task) => {
            send_task.abort();
        },
    }
}

pub async fn ws_handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    return ws.on_upgrade(async move |socket| handle_socket(state, socket).await);
}
