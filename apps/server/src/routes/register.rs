use axum::{
    debug_handler,
    extract::State,
    http::{status::StatusCode, Response},
    Json,
};

use crate::{
    database::{
        login::{get_user, LoginInfo},
        register::{add_user, UserCreation},
    },
    errors::conflict::BasicResponseError,
    state::AppState,
    util::user::PasswordlessActiveUser,
};

#[debug_handler]
pub async fn register(
    State(state): State<AppState>,
    Json(user): Json<UserCreation>,
) -> Result<Response<String>, Response<String>> {
    let client = state.client;

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

        let mut res = Response::new(serde_json::to_string(&conflict_error).unwrap());

        let s = res.status_mut();
        *s = StatusCode::CONFLICT;

        return Err(res);
    }

    println!("Making new user");

    let new_user = add_user(&client, user).await.unwrap();
    let user = PasswordlessActiveUser::from_active(new_user);

    let json = Response::new(serde_json::to_string(&user).unwrap());

    return Ok(json);
}
