use deadpool_postgres::Client;
use futures::executor::block_on;
use mayhem_db::models::user::User;
use rocket::serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

pub async fn get_user(
    client: &Client,
    user_info: &LoginInfo,
) -> Result<User, AppError> {
    let _stmt = include_str!("../sql/users/get.sql");
    let _stmt = _stmt.replace("$username", user_info.username.as_str());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let res = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| block_on(User::from_postgres_ref(row, client)).unwrap())
        .collect::<Vec<User>>()
        .pop();

    match res {
        Some(user) => Ok(user),
        None => Err(AppError::NotFound),
    }
}
