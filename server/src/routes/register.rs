use rocket::{put, response::status, serde::json::Json};

use crate::{
    database::{
        login::{get_user, LoginInfo},
        register::add_user,
    },
    errors::conflict::{BasicResponseError, RealConflictError},
    models::{full_user::RealFullUser, user::RawUserCreation},
    state::DatabaseProvider,
};

#[put("/api/users", data = "<user>")]
pub async fn register(
    db: &DatabaseProvider,
    user: Json<RawUserCreation<'_>>,
) -> Result<Json<RealFullUser>, status::Conflict<Json<RealConflictError>>> {
    let client = db.get().await.unwrap();

    let user_info_check = LoginInfo {
        username: user.username.clone(),
        password: user.password.clone(),
    };

    let existing = get_user(&client, &user_info_check).await;

    if existing.is_ok() {
        let conflict_error = BasicResponseError {
            code: 409,
            message: "User already exists!",
        };

        let res = status::Conflict(Some(Json(RealConflictError::from(conflict_error))));

        return Err(res);
    }

    let new_user = add_user(&client, user.into_inner()).await.unwrap();
    let json = Json(new_user);

    return Ok(json);
}
