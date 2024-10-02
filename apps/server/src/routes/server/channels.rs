use crate::{state::AppState, ws::handlers::channel::ChannelsData};
use axum::{
    debug_handler,
    extract::{Path, State},
    http::{HeaderMap, Response, StatusCode},
};

#[debug_handler]
pub async fn channels(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(server_id): Path<i32>,
) -> Response<String> {
    todo!()
}
