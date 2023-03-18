use crate::errors::AppError;

use mayhem_db::{
    models::{user, user_settings},
    sea_orm::{ActiveModelTrait, DbConn, ModelTrait, Set},
    util::{CompleteServer, CompleteUser},
    Client,
};

use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};

use std::sync::Arc;

pub fn generate_token() -> String {
    return Alphanumeric.sample_string(&mut thread_rng(), 128);
}

pub async fn add_token(
    client: &Arc<Client>,
    user_info: &CompleteUser,
) -> Result<CompleteUser, AppError> {
    let res = client
        .query
        .user
        .find_user_by_id(user_info.id)
        .await
        .unwrap();

    match res {
        Some(user) => {
            let mut user: user::ActiveModel = user.into();

            user.token = Set(Some(generate_token()));

            let user = user.update(&client.client as &DbConn).await;

            match user {
                Ok(user) => {
                    let servers_result = client.query.user.find_user_servers(&user).await;

                    let servers: Vec<CompleteServer> = match servers_result {
                        Ok(res) => res,
                        Err(_) => return Err(AppError::NotFound),
                    };

                    let settings = user
                        .find_related(user_settings::Entity)
                        .one(&client.client as &DbConn)
                        .await
                        .unwrap();

                    return Ok(CompleteUser {
                        id: user.id,
                        first_name: user.first_name,
                        last_name: user.last_name,
                        email: user.email,
                        username: user.username,
                        password: user.password,
                        servers,
                        settings,
                        token: user.token,
                    });
                }

                Err(_) => Err(AppError::NotFound),
            }
        }

        None => Err(AppError::NotFound),
    }
}
