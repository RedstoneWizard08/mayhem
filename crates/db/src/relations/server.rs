use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use crate::{
    models::{Role, Server, User},
    schema::{member_roles, roles},
    Result,
};

impl Server {
    pub async fn get_roles(&self, pool: &Pool<AsyncPgConnection>) -> Result<Vec<Role>> {
        // For some reason, Diesel doesn't want to implement `BelongingToDsl` for `Role`.
        // I have no idea why. I've checked issues and discussions, and... nothing.
        // TODO: Get it to work.

        // Role::belonging_to(&self)
        //     .select(Role::as_select())
        //     .load(&mut pool.get().await?)
        //     .await?

        Ok(roles::table
            .filter(roles::server_id.eq(self.id.unwrap_or(-1)))
            .select(Role::as_select())
            .load(&mut pool.get().await?)
            .await?)
    }

    pub async fn get_roles_for_user(
        &self,
        user: &User,
        pool: &Pool<AsyncPgConnection>,
    ) -> Result<Vec<Role>> {
        Ok(member_roles::table
            .inner_join(roles::table)
            .filter(member_roles::user_id.eq(user.id.unwrap_or(-1)))
            .filter(roles::server_id.eq(self.id.unwrap_or(-1)))
            .select(Role::as_select())
            .load(&mut pool.get().await?)
            .await?)
    }
}
