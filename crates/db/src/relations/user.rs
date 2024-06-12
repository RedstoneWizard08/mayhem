use crate::{
    models::{Server, User},
    schema::{server_members, servers},
    Result,
};

use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

impl User {
    pub async fn get_servers(&self, pool: &Pool<AsyncPgConnection>) -> Result<Vec<Server>> {
        // For some reason, Diesel doesn't want to implement `BelongingToDsl` for `ServerMember`.
        // I have no idea why. I've checked issues and discussions, and... nothing.
        // TODO: Get it to work.

        // ServerMember::belonging_to(&self)
        //     .inner_join(servers::table)
        //     .select(Server::as_select())
        //     .load(&mut pool.get().await?)
        //     .await?

        Ok(server_members::table
            .inner_join(servers::table)
            .filter(server_members::user_id.eq(self.id.unwrap_or(-1)))
            .select(Server::as_select())
            .load(&mut pool.get().await?)
            .await?)
    }
}
