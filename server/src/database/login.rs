use deadpool_postgres::Client;
use rocket::serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
    errors::AppError,
    models::{
        full_user::{FullUser, RealFullUser},
        user::{RealUserWithId, UserWithId},
        user_settings::UserSettingsWithId,
    },
};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginInfo<'r> {
    pub username: &'r str,
    pub password: &'r str,
}

pub async fn get_user<'r>(
    client: &Client,
    user_info: &LoginInfo<'r>,
) -> Result<RealFullUser, AppError> {
    let _stmt = include_str!("../sql/users/get.sql");
    let _stmt = _stmt.replace("$username", user_info.username);
    let stmt = client.prepare(&_stmt).await.unwrap();

    let res = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| RealUserWithId::from_row_ref(row).unwrap())
        .collect::<Vec<RealUserWithId>>()
        .pop();

    let user_find = res.clone();
    let user: RealUserWithId;

    match user_find {
        Some(user_) => user = user_,
        None => return Err(AppError::NotFound),
    }

    let _stmt2 = include_str!("../sql/users/get_settings.sql");
    let _stmt2 = _stmt2.replace("$user_id", user.id.to_string().as_str());
    let stmt2 = client.prepare(&_stmt2).await.unwrap();

    let res2 = client
        .query(&stmt2, &[])
        .await?
        .iter()
        .map(|row| UserSettingsWithId::from_row_ref(row).unwrap())
        .collect::<Vec<UserSettingsWithId>>()
        .pop();

    if res2.is_some() {
        let full = FullUser::new(
            UserWithId {
                id: user.id,
                first_name: user.first_name.as_str(),
                last_name: user.last_name.as_str(),
                email: user.email.as_str(),
                username: user.username.as_str(),
                password: user.password.as_str(),
            },
            res2.unwrap(),
        );

        return Ok(RealFullUser::from(full));
    } else {
        return Err(AppError::NotFound);
    }
}
