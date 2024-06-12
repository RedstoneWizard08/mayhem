use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use crate::{
    models::{Friend, User},
    schema::users,
    Result,
};

impl Friend {
    pub async fn get_user(&self, pool: &Pool<AsyncPgConnection>) -> Result<User> {
        Ok(users::table
            .filter(users::id.eq(self.user_id))
            .select(User::as_select())
            .get_result(&mut pool.get().await?)
            .await?)
    }

    pub async fn get_friend(&self, pool: &Pool<AsyncPgConnection>) -> Result<User> {
        Ok(users::table
            .filter(users::id.eq(self.friend_id))
            .select(User::as_select())
            .get_result(&mut pool.get().await?)
            .await?)
    }
}
