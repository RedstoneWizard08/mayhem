use axum::{
    debug_handler,
    extract::State,
    http::{status, Response},
    response::Result,
    Json,
};

use mayhem_db::models::User;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(user_info): Json<LoginInfo>,
) -> Result<User, String> {
    User::login(&user_info.username, &user_info.password, &state.pool)
        .await
        .map_err(|v| format!("{}", v))
}

#[debug_handler]
pub async fn token_login(
    State(state): State<AppState>,
    Json(token_info): Json<Value>,
) -> Result<Response<String>, Response<String>> {
    todo!()
}
