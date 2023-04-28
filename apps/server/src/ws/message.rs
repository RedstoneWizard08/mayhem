use std::sync::Arc;

use crate::state::AppState;
use axum::extract::ws::{Message, WebSocket};
use futures::stream::SplitSink;
use tokio::sync::{broadcast::Sender, Mutex};

use super::handlers::finder::find_handler;

pub async fn handle_message(
    message: Message,
    state: AppState,
    wtx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    sender: Sender<String>,
) {
    if let Message::Text(text) = message {
        let client = &state.client;
        let db = &client.client;

        find_handler(text.as_str(), db, wtx, sender).await;
    }
}
