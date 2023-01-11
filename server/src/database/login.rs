use mayhem_db::{util::CompleteUser, Client};
use rocket::{
    serde::{Deserialize, Serialize},
    State,
};

use crate::errors::AppError;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

pub async fn get_user(
    client: &State<Client>,
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
