pub mod client;
pub mod error;
pub mod index;
pub mod login;
pub mod register;
pub mod server;

use axum::{
    body::Body,
    routing::{get, post, put},
    Router,
};

pub use client::handle_client_proxy as client_handler;
pub use error::not_found as handle_error;
pub use index::index as handle_index;
pub use login::login as handle_login;
pub use register::register as handle_register;

use crate::state::AppState;

pub fn register(router: Router<AppState, Body>) -> Router<AppState, Body> {
    let router = router
        .route("/", get(handle_index))
        .route("/api/users", post(handle_login))
        .route("/api/users", put(handle_register));

    let router = server::register(router);

    return router;
}
