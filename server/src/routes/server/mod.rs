pub mod index;

use axum::{body::Body, routing::get, Router};
use index::index as index_handler;

use crate::state::AppState;

pub fn register(router: Router<AppState, Body>) -> Router<AppState, Body> {
    return router.route("/api/v1/server", get(index_handler));
}
