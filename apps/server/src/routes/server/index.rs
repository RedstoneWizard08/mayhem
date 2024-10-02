use crate::{state::AppState, ws::handlers::server::ServersData};

use axum::{
    debug_handler,
    extract::State,
    http::{HeaderMap, Response, StatusCode},
};

#[debug_handler]
pub async fn index(State(state): State<AppState>, headers: HeaderMap) -> Response<String> {
    todo!()
}
