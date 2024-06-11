use crate::{
    database::{
        login::{get_user, LoginInfo},
        token::add_token,
    },
    errors::conflict::BasicResponseError,
    state::AppState,
};

use axum::{debug_handler, extract::State, response::Response, Json};
use http::status;

use pbkdf2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Pbkdf2,
};

#[debug_handler]
pub async fn get_token(
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
                Ok(_) => {
                    let updated = add_token(&db, &user).await;

                    match updated {
                        Ok(user) => Ok(Response::new(user.token.unwrap())),

                        Err(_) => {
                            let mut resp = Response::new(
                                serde_json::to_string(&BasicResponseError {
                                    code: 500,
                                    message: "Could not update the user!".to_string(),
                                })
                                .unwrap(),
                            );

                            let s = resp.status_mut();
                            *s = status::StatusCode::from_u16(500).unwrap();

                            return Err(resp);
                        }
                    }
                }

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
