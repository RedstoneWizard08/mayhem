use anyhow::Result;
use mayhem_db::diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};

use crate::config::{AppConfig, DatabaseConfig};

pub fn create_connection_url(config: &DatabaseConfig) -> String {
    let mut prefix = String::new();

    if let Some(user) = config.user {
        prefix = user;

        if let Some(pass) = config.pass {
            prefix = format!("{}:{}", prefix, pass);
        }

        prefix = format!("{}@", prefix);
    }

    return format!(
        "postgresql://{}{}:{}/{}",
        prefix, config.host, config.port, config.database
    );
}

pub fn connect(config: &DatabaseConfig) -> Result<Pool<AsyncPgConnection>> {
    Ok(mayhem_db::connect(create_connection_url(config))?)
}
