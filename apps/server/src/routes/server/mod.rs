pub mod channels;
pub mod index;
pub mod messages;

use axum::{body::Body, routing::get, Router};
use channels::channels as channel_handler;
use index::index as index_handler;
use messages::messages as messages_handler;

use crate::state::AppState;

pub fn register(router: Router<AppState, Body>) -> Router<AppState, Body> {
    return router
        .route("/api/v1/servers", get(index_handler))
        .route("/api/v1/servers/:server_id/channels", get(channel_handler))
        .route(
            "/api/v1/servers/:server_id/channels/:channel_id/messages",
            get(messages_handler),
        );
}
