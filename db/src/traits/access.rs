use async_trait::async_trait;
use tokio_postgres::{Client, Error, Row};

#[async_trait]
pub trait PostgresAccessible {
    async fn from_postgres(row: Row, client: &Client) -> Self;
    async fn update(&self, client: &Client) -> Result<Vec<Row>, Error>;
}
