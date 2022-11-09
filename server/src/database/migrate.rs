use deadpool_postgres::Client;
use tokio_postgres::Statement;

use crate::logging::warn;

pub async fn run_migrations(client: &Client) {
    let _stmt = include_str!("../sql/migrations/schema.sql");
    let _stmt = _stmt
        .split(";")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut statements: Vec<Statement> = Vec::new();

    for item in _stmt {
        statements.push(client.prepare((item + ";").as_str()).await.unwrap());
    }

    for item in statements {
        client.query(&item, &[]).await.unwrap_or_else(move |_| {
            warn("Could not run migrations!");

            return Vec::new();
        });
    }
}
