use mayhem_db::models::user::User;
use rocket::{put, response::status, serde::json::Json};

use crate::{
    database::{
        login::{get_user, LoginInfo},
        register::{add_user, UserCreation},
    },
    errors::conflict::BasicResponseError,
    state::DatabaseProvider,
};

#[put("/api/users", data = "<user>")]
pub async fn register(
    db: &DatabaseProvider,
    user: Json<UserCreation>,
) -> Result<Json<User>, status::Conflict<Json<BasicResponseError>>> {
    let client = db.get().await.unwrap();

    let user_info_check = LoginInfo {
        username: user.username.clone(),
        password: user.password.clone(),
    };

    let existing = get_user(&client, &user_info_check).await;

    if existing.is_ok() {
        let conflict_error = BasicResponseError {
            code: 409,
            message: "User already exists!".to_string(),
        };

        let res = status::Conflict(Some(Json(BasicResponseError::from(conflict_error))));

        return Err(res);
    }

    println!("Making new user");
    let new_user = add_user(&client, user.into_inner()).await.unwrap();
    let json = Json(new_user);

    return Ok(json);
}
