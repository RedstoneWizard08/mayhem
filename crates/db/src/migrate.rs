use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use migrations::{embed_migrations, EmbeddedMigrations};

use crate::Result;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../migrations");

pub async fn migrate(pool: &Pool<AsyncPgConnection>) -> Result<()> {
    MIGRATIONS
        .run_pending_migrations(&mut pool.get().await?)
        .await?;

    Ok(())
}
