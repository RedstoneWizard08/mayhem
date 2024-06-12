use std::env;

use serde::{Deserialize, Serialize};

use super::env::make_env_key;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: i32,
    pub user: Option<String>,
    pub pass: Option<String>,
    pub database: String,
}

impl DatabaseConfig {
    pub fn parse_from_env(prefix: &str) -> DatabaseConfig {
        let db_prefix_s = format!("{}_DB", prefix);
        let db_prefix = db_prefix_s.as_str();
        let mut config = DatabaseConfig::default();

        let env_host = env::var(make_env_key(db_prefix, "HOST"));
        let env_port = env::var(make_env_key(db_prefix, "PORT"));
        let env_user = env::var(make_env_key(db_prefix, "USER"));
        let env_pass = env::var(make_env_key(db_prefix, "PASS"));
        let env_database = env::var(make_env_key(db_prefix, "DATABASE"));

        if let Ok(host) = env_host {
            config.host = host;
        }

        if let Ok(port) = env_port {
            config.port = port.parse::<i32>().unwrap_or(config.port);
        }

        if let Ok(user) = env_user {
            config.user = Some(user);
        }

        if let Ok(pass) = env_pass {
            config.pass = Some(pass);
        }

        if let Ok(database) = env_database {
            config.database = database;
        }

        return config;
    }

    pub fn layer(first: DatabaseConfig, second: DatabaseConfig) -> DatabaseConfig {
        let mut config = first.clone();
        let default = DatabaseConfig::default();

        if first.host == default.host {
            config.host = second.host;
        }

        if first.port == default.port {
            config.port = second.port;
        }

        if first.user == default.user {
            config.user = second.user;
        }

        if first.pass == default.pass {
            config.pass = second.pass;
        }

        if first.database == default.database {
            config.database = second.database;
        }

        return config;
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        return Self {
            host: "127.0.0.1".to_string(),
            port: 5432,
            user: None,
            pass: None,
            database: "postgres".to_string(),
        };
    }
}
