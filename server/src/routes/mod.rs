pub mod error;
pub mod index;
pub mod login;
pub mod register;
pub mod server;

use std::sync::Arc;

use axum::{
    body::Body,
    routing::{get, post, put},
    Router,
};

pub use error::not_found as handle_error;
pub use index::index as handle_index;
pub use login::login as handle_login;
use mayhem_db::Client;
pub use register::register as handle_register;

pub fn register(router: Router<Arc<Client>, Body>) -> Router<Arc<Client>, Body> {
    let router = router
        .route("/", get(handle_index))
        .route("/api/users", post(handle_login))
        .route("/api/users", put(handle_register));

    let router = server::register(router);

    return router;
}
