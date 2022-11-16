use mayhem_db::models::user::User;
use pbkdf2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Pbkdf2,
};
use rocket::{post, serde::json::Json, response::status};
use rocket_db_pools::Connection;

use crate::{
    database::login::{get_user, LoginInfo},
    state::DatabaseProvider, errors::conflict::BasicResponseError,
};

#[post("/api/users", data = "<info>")]
pub async fn login(
    db: Connection<DatabaseProvider>,
    info: Json<LoginInfo>,
) -> Result<
    Result<
        Json<User>,
        status::Unauthorized<Json<BasicResponseError>>,
    >,
    status::NotFound<Json<BasicResponseError>>,
> {
    let user_info = info.into_inner();
    let user_get = get_user(&db, &user_info).await;

    match user_get {
        Ok(user) => {
            let password_from_db = user.password.clone();
            let parsed_hash = PasswordHash::new(&password_from_db).unwrap();

            let valid = Pbkdf2
                .verify_password(user_info.password.clone().as_bytes(), &parsed_hash);
            
            match valid {
                Ok(_) => {
                    return Ok(Ok(Json(user)));
                },
                Err(_) => {
                    return Ok(
                        Err(
                            status::Unauthorized(
                                Some(
                                    Json(
                                        BasicResponseError {
                                            code: 401,
                                            message: "Invalid password!".to_string(),
                                        }
                                    )
                                )
                            )
                        )
                    );
                }
            }
        },
        Err(_) => {
            return Err(
                status::NotFound(
                    Json(
                        BasicResponseError {
                            code: 404,
                            message: "User not found!".to_string(),
                        }
                    )
                )
            );
        }
    }
}
