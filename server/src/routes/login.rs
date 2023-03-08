use axum::{
    debug_handler,
    extract::State,
    http::{status, Response},
    response::Result,
    Json,
};

use pbkdf2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Pbkdf2,
};

use crate::{
    database::login::{get_user, LoginInfo},
    errors::conflict::BasicResponseError,
    util::user::PasswordlessUser, state::AppState,
};

#[debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(user_info): Json<LoginInfo>,
) -> Result<Response<String>, Response<String>> {
    let db = state.client;
    let user_get = get_user(&db, &user_info).await;

    match user_get {
        Ok(user) => {
            let password_from_db = user.password.clone();
            let parsed_hash = PasswordHash::new(&password_from_db).unwrap();

            let valid = Pbkdf2.verify_password(user_info.password.clone().as_bytes(), &parsed_hash);

            match valid {
                Ok(_) => Ok(Response::new(
                    serde_json::to_string(&PasswordlessUser::from_complete(user)).unwrap(),
                )),

                Err(_) => {
                    let mut resp = Response::new(
                        serde_json::to_string(&BasicResponseError {
                            code: 401,
                            message: "Invalid password!".to_string(),
                        })
                        .unwrap(),
                    );

                    let s = resp.status_mut();
                    *s = status::StatusCode::UNAUTHORIZED;

                    return Err(resp);
                }
            }
        }
        Err(_) => {
            let mut resp = Response::new(
                serde_json::to_string(&BasicResponseError {
                    code: 400,
                    message: "User not found!".to_string(),
                })
                .unwrap(),
            );

            let s = resp.status_mut();
            *s = status::StatusCode::BAD_REQUEST;

            return Err(resp);
        }
    }
}
