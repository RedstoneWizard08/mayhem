use rocket_db_pools::{deadpool_postgres, Database};

#[derive(Database)]
#[database("dev_db")]
pub struct DatabaseProvider(deadpool_postgres::Pool);
