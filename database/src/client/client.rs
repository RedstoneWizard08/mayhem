use std::sync::Arc;

use migration::{Migrator, MigratorTrait, DbErr};
use sea_orm::DbConn;

use super::{
    connector::{connect, DatabaseConnectionOptions},
    insertion::DatabaseInsertionHelper,
    mutation::DatabaseMutationHelper,
    query::DatabaseQueryHelper,
    removal::DatabaseRemovalHelper,
};

#[derive(Clone)]
pub struct Client {
    pub opts: Option<DatabaseConnectionOptions>,
    pub client: Arc<DbConn>,
    pub inserter: DatabaseInsertionHelper,
    pub mutator: DatabaseMutationHelper,
    pub remover: DatabaseRemovalHelper,
    pub query: DatabaseQueryHelper,
}

unsafe impl Sync for Client {}
unsafe impl Send for Client {}

impl Client {
    pub async fn connect(opts: DatabaseConnectionOptions) -> Self {
        return Self::of(connect(opts.clone()).await.unwrap());
    }

    pub fn of(conn: DbConn) -> Self {
        let conn = Arc::new(conn);

        let inserter = DatabaseInsertionHelper::create(conn.clone());
        let mutator = DatabaseMutationHelper::create(conn.clone());
        let remover = DatabaseRemovalHelper::create(conn.clone());
        let query = DatabaseQueryHelper::create(conn.clone());

        return Self {
            opts: None,
            client: conn,
            inserter,
            mutator,
            remover,
            query,
        };
    }

    pub async fn run_migrations(&self) -> Result<(), DbErr> {
        return Migrator::up(&self.client, None).await;
    }
}
