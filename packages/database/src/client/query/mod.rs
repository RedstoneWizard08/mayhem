use sea_orm::DbConn;
use std::sync::Arc;

use self::{server::ServerQueryHelper, user::UserQueryHelper};

pub mod common;
pub mod server;
pub mod user;

#[derive(Clone)]
pub struct QueryHelper {
    pub user: UserQueryHelper,
    pub server: ServerQueryHelper,
}

unsafe impl Sync for QueryHelper {}
unsafe impl Send for QueryHelper {}

impl QueryHelper {
    pub fn create(client: Arc<DbConn>) -> Self {
        let server = ServerQueryHelper {
            client: client.clone(),
        };

        let user = UserQueryHelper {
            client,
            server: Arc::new(server.clone()),
        };

        return Self { user, server };
    }
}
