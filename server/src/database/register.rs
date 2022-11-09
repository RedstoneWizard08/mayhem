use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
    errors::AppError,
    models::{
        full_user::{FullUser, RealFullUser},
        user::{RawUserCreation, RealUserWithId, UserWithId},
        user_settings::UserSettingsWithId,
    },
    util::password::hash,
};

pub async fn add_user<'r>(
    client: &Client,
    user_info: RawUserCreation<'r>,
) -> Result<RealFullUser, AppError> {
    let _stmt = include_str!("../sql/users/add.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();

    let password = hash(&user_info.password);

    let res = client
        .query(
            &stmt,
            &[
                &user_info.first_name,
                &user_info.last_name,
                &user_info.email,
                &password,
                &user_info.username,
            ],
        )
        .await?
        .iter()
        .map(|row| RealUserWithId::from_row_ref(row).unwrap())
        .collect::<Vec<RealUserWithId>>()
        .pop();

    let user = res.clone().unwrap();

    let _stmt2 = include_str!("../sql/users/link_settings.sql");
    let stmt2 = client.prepare(&_stmt2).await.unwrap();

    let res2 = client
        .query(&stmt2, &[&user.id])
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
