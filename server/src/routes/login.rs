use mayhem_db::Client;
use pbkdf2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Pbkdf2,
};
use rocket::{post, response::status, serde::json::Json, State};

use crate::{
    database::login::{get_user, LoginInfo},
    errors::conflict::BasicResponseError,
    util::user::PasswordlessUser,
};

#[post("/api/users", data = "<info>")]
pub async fn login(
    db: &State<Client>,
    info: Json<LoginInfo>,
) -> Result<
    Result<Json<PasswordlessUser>, status::Unauthorized<Json<BasicResponseError>>>,
    status::NotFound<Json<BasicResponseError>>,
> {
    let user_info = info.into_inner();
    let user_get = get_user(&db, &user_info).await;

    match user_get {
        Ok(user) => {
            let password_from_db = user.password.clone();
            let parsed_hash = PasswordHash::new(&password_from_db).unwrap();

            let valid = Pbkdf2.verify_password(user_info.password.clone().as_bytes(), &parsed_hash);

            match valid {
                Ok(_) => Ok(Ok(Json(PasswordlessUser::from_complete(user)))),

                Err(_) => Ok(Err(status::Unauthorized(Some(Json(BasicResponseError {
                    code: 401,
                    message: "Invalid password!".to_string(),
                }))))),
            }
        }
        Err(_) => {
            return Err(status::NotFound(Json(BasicResponseError {
                code: 404,
                message: "User not found!".to_string(),
            })));
        }
    }
}
