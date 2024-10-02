use crate::state::AppState;
use axum::{
    debug_handler,
    extract::{Path, State},
    http::{HeaderMap, Response, StatusCode},
};

#[debug_handler]
pub async fn messages(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((_, channel_id)): Path<(i32, i32)>,
) -> Response<String> {
    todo!()
}
