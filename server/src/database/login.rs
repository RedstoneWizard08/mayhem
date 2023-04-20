use std::sync::Arc;

use mayhem_db::{util::CompleteUser, Client};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TokenInfo {
    pub token: String,
}

pub async fn get_user(
    client: &Arc<Client>,
    user_info: &LoginInfo,
) -> Result<CompleteUser, AppError> {
    let res = client
        .query
        .user
        .find_user_by_name(user_info.username.clone())
        .await
        .unwrap();

    match res {
        Some(user) => Ok(user),
        None => Err(AppError::NotFound),
    }
}

pub async fn get_user_by_token(
    client: &Arc<Client>,
    token_info: &TokenInfo,
) -> Result<CompleteUser, AppError> {
    let res = client
        .query
        .user
        .find_user_by_token(token_info.token.clone())
        .await
        .unwrap();

    match res {
        Some(user) => Ok(user),
        None => Err(AppError::NotFound),
    }
}
