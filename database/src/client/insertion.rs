use crate::models::user;
use migration::DbErr;
use sea_orm::{ActiveModelTrait, DbConn, Set};
use std::sync::Arc;

#[derive(Clone)]
pub struct DatabaseInsertionHelper {
    client: Arc<DbConn>,
}

unsafe impl Sync for DatabaseInsertionHelper {}
unsafe impl Send for DatabaseInsertionHelper {}

impl DatabaseInsertionHelper {
    pub fn create(client: Arc<DbConn>) -> Self {
        return Self { client };
    }

    pub async fn create_user(&self, data: user::Model) -> Result<user::ActiveModel, DbErr> {
        return (user::ActiveModel {
            first_name: Set(data.first_name),
            last_name: Set(data.last_name),
            email: Set(data.email),
            username: Set(data.username),
            password: Set(data.password),
            ..Default::default()
        })
        .save(&self.client as &DbConn)
        .await;
    }
}
