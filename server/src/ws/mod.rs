pub mod store;
pub mod handler;
pub mod handlers;
pub mod message;

use axum::{Router, body::Body, routing::get};

use crate::state::AppState;

pub use self::handler::ws_handler;
pub use store::Room as ChatRoom;

pub fn register(router: Router<AppState, Body>) -> Router<AppState, Body> {
    return router.route("/api/ws", get(ws_handler));
}
