use crate::models::user;
use migration::DbErr;
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait, Set};
use std::sync::Arc;

#[derive(Clone)]
pub struct MutationHelper {
    client: Arc<DbConn>,
}

unsafe impl Sync for MutationHelper {}
unsafe impl Send for MutationHelper {}

impl MutationHelper {
    pub fn create(client: Arc<DbConn>) -> Self {
        return Self { client };
    }

    pub async fn update_user(&self, id: i32, data: user::Model) -> Result<user::Model, DbErr> {
        let object: user::ActiveModel = user::Entity::find_by_id(id)
            .one(&self.client as &DbConn)
            .await
            .unwrap()
            .ok_or(DbErr::Custom("Cannot find model!".to_owned()))
            .map(Into::into)
            .unwrap();

        return (user::ActiveModel {
            id: object.id,
            first_name: Set(data.first_name),
            last_name: Set(data.last_name),
            email: Set(data.email),
            username: Set(data.username),
            password: Set(data.password),
            token: Set(None),
        })
        .update(&self.client as &DbConn)
        .await;
    }
}
