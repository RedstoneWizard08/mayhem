use std::sync::Arc;

use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::DbConn;

use super::{
    connector::{connect, ConnectionOptions},
    insertion::InsertionHelper,
    mutation::MutationHelper,
    query::QueryHelper,
    removal::RemovalHelper,
};

#[derive(Clone)]
pub struct Client {
    pub opts: Option<ConnectionOptions>,
    pub client: Arc<DbConn>,
    pub inserter: InsertionHelper,
    pub mutator: MutationHelper,
    pub remover: RemovalHelper,
    pub query: QueryHelper,
}

unsafe impl Sync for Client {}
unsafe impl Send for Client {}

impl Client {
    pub async fn connect(opts: ConnectionOptions) -> Self {
        return Self::of(connect(opts.clone()).await.unwrap());
    }

    pub fn of(conn: DbConn) -> Self {
        let conn = Arc::new(conn);

        let inserter = InsertionHelper::create(conn.clone());
        let mutator = MutationHelper::create(conn.clone());
        let remover = RemovalHelper::create(conn.clone());
        let query = QueryHelper::create(conn.clone());

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
