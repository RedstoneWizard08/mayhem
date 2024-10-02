use axum::{
    debug_handler,
    extract::{Path, State},
    response::Response,
};

use crate::state::AppState;

#[debug_handler]
pub async fn user(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<String, Response<String>> {
    todo!()
}
