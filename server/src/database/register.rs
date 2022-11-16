use deadpool_postgres::Client;
use futures::executor::block_on;
use mayhem_db::models::user::User;
use rocket::serde::{Serialize, Deserialize};

use crate::{
    errors::AppError,
    util::password::hash,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UserCreation {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

pub async fn add_user(
    client: &Client,
    user_info: UserCreation,
) -> Result<User, AppError> {
    let _stmt = include_str!("../sql/users/add.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();

    let password = hash(&user_info.password);
    let servers: Vec<i32> = Vec::new();

    let user = client
        .query(
            &stmt,
            &[
                &user_info.first_name,
                &user_info.last_name,
                &user_info.email,
                &password,
                &user_info.username,
                &servers,
            ],
        )
        .await?
        .iter()
        .map(|row| block_on(User::from_postgres_ref(row.to_owned(), client)).unwrap())
        .collect::<Vec<User>>()
        .pop();

    if user.is_some() {
        return Ok(user.unwrap());
    } else {
        return Err(AppError::NotFound);
    }
}
