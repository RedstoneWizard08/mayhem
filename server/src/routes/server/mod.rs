pub mod index;

use std::sync::Arc;

use axum::{body::Body, routing::get, Router};
use index::index as index_handler;
use mayhem_db::Client;

pub fn register(router: Router<Arc<Client>, Body>) -> Router<Arc<Client>, Body> {
    return router.route("/api/v1/server", get(index_handler));
}
