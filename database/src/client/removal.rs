use crate::models::user;
use migration::DbErr;
use sea_orm::{ActiveModelTrait, DbConn, DeleteResult, EntityTrait};
use std::sync::Arc;

#[derive(Clone)]
pub struct RemovalHelper {
    client: Arc<DbConn>,
}

unsafe impl Sync for RemovalHelper {}
unsafe impl Send for RemovalHelper {}

impl RemovalHelper {
    pub fn create(client: Arc<DbConn>) -> Self {
        return Self { client };
    }

    pub async fn delete_user(&self, id: i32) -> Result<DeleteResult, DbErr> {
        let object: user::ActiveModel = user::Entity::find_by_id(id)
            .one(&self.client as &DbConn)
            .await
            .unwrap()
            .ok_or(DbErr::Custom("Cannot find model!".to_owned()))
            .map(Into::into)
            .unwrap();

        return object.delete(&self.client as &DbConn).await;
    }

    pub async fn delete_all_users(&self) -> Result<DeleteResult, DbErr> {
        return user::Entity::delete_many()
            .exec(&self.client as &DbConn)
            .await;
    }
}
