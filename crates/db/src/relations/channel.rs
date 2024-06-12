use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use crate::{
    models::{Channel, Server},
    schema::servers,
    Result,
};

impl Channel {
    pub async fn get_server(&self, pool: &Pool<AsyncPgConnection>) -> Result<Server> {
        Ok(servers::table
            .filter(servers::id.eq(self.server_id))
            .select(Server::as_select())
            .get_result(&mut pool.get().await?)
            .await?)
    }
}
