use mayhem_db::{Client, models::user::{ActiveModel as User, Model as UserModel}};
use rocket::{serde::{Deserialize, Serialize}, State};

use crate::{errors::AppError, util::password::hash};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UserCreation {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

pub async fn add_user(client: &State<Client>, user_info: UserCreation) -> Result<User, AppError> {
    let password = hash(&user_info.password);

    let user = client.inserter.create_user(UserModel {
        first_name: user_info.first_name,
        last_name: user_info.last_name,
        email: user_info.email,
        username: user_info.username,
        password,
        id: -1,
    }).await;

    if user.is_ok() {
        return Ok(user.unwrap());
    } else {
        return Err(AppError::NotFound);
    }
}
