use crate::state::AppState;

use axum::{debug_handler, extract::State, response::Response, Json};
use serde_json::Value;

#[debug_handler]
pub async fn get_token(
    State(state): State<AppState>,
    Json(user_info): Json<Value>,
) -> Result<Response<String>, Response<String>> {
    todo!()
}
