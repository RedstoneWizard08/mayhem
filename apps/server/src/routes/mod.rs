pub mod client;
pub mod error;
pub mod login;
pub mod register;
pub mod server;
pub mod token;
pub mod user;

use axum::{
    body::Body,
    routing::{get, post, put},
    Router,
};

pub use client::handle_client_proxy as client_handler;
pub use error::not_found as handle_error;
pub use login::login as handle_login;
pub use login::token_login as handle_token_login;
pub use register::register as handle_register;
pub use token::get_token as handle_token;
pub use user::user as handle_user;

use crate::state::AppState;

pub fn register(router: Router<AppState, Body>) -> Router<AppState, Body> {
    let router = router
        .route("/api/users", post(handle_login))
        .route("/api/users/token", post(handle_token_login))
        .route("/api/users", put(handle_register))
        .route("/api/token", post(handle_token))
        .route("/api/users/:user", get(handle_user));

    let router = server::register(router);

    return router;
}
