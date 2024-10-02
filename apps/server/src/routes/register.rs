use axum::{
    debug_handler,
    extract::State,
    http::{status::StatusCode, Response},
    Json,
};
use mayhem_db::models::{Birthday, User};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterInfo {
    pub username: String,
    pub password: String,
    pub email: String,
    pub birthday: Birthday,
}

#[debug_handler]
pub async fn register(
    State(state): State<AppState>,
    Json(info): Json<RegisterInfo>,
) -> Result<User, String> {
    User::create(
        &info.username,
        &info.email,
        &info.password,
        &info.birthday,
        &state.pool,
    )
    .await
    .map_err(|err| format!("{}", err))
}
