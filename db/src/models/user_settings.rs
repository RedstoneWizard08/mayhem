use async_trait::async_trait;
use rocket::serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error, Row, Statement};

use crate::traits::access::PostgresAccessible;

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(crate = "rocket::serde")]
pub struct UserSettings {
    pub id: i32,
    pub user_id: i32,
}

#[async_trait]
impl PostgresAccessible for UserSettings {
    async fn from_postgres(row: Row, _client: &Client) -> Self {
        println!("Getting setting props...");
        let id = row.get("id");
        let user_id = row.get("user_id");

        println!("Making settings...");
        return Self { id, user_id };
    }

    async fn update(&self, client: &Client) -> Result<Vec<Row>, Error> {
        let statement_raw = include_str!("../sql/insert_user_settings.sql");
        let statement_result = client.prepare(statement_raw).await;
        let statement_out: Statement;

        match statement_result {
            Ok(statement) => statement_out = statement,
            Err(err) => return Err(err),
        };

        let query_result = client.query(&statement_out, &[&self.user_id]).await;

        let query_out: Vec<Row>;

        match query_result {
            Ok(rows) => query_out = rows,
            Err(err) => return Err(err),
        };

        return Ok(query_out);
    }
}
