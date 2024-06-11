use axum::{
    debug_handler,
    extract::{Path, State},
    response::Response,
};
use http::status;

use crate::{errors::conflict::BasicResponseError, state::AppState, util::user::PasswordlessUser};

#[debug_handler]
pub async fn user(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<String, Response<String>> {
    let user = state
        .client
        .query
        .user
        .find_user_by_id(user_id)
        .await
        .unwrap();

    if let Some(user) = user {
        let user = state
            .client
            .query
            .user
            .finish_user(user)
            .await
            .unwrap()
            .unwrap();
        return Ok(serde_json::to_string(&PasswordlessUser::from_complete(user)).unwrap());
    }

    let mut resp = Response::new(
        serde_json::to_string(&BasicResponseError {
            code: 400,
            message: "Invalid user!".to_string(),
        })
        .unwrap(),
    );

    let s = resp.status_mut();
    *s = status::StatusCode::BAD_REQUEST;

    return Err(resp);
}
