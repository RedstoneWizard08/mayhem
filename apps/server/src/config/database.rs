use std::env;

use serde::{Deserialize, Serialize};

use super::env::make_env_key;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DatabaseConfig {
    pub protocol: String,
    pub host: String,
    pub port: i32,
    pub user: Option<String>,
    pub pass: Option<String>,
    pub database: String,

    pub min_connections: i32,
    pub max_connections: i32,
    pub connect_timeout: i32,
    pub idle_timeout: i32,
}

impl DatabaseConfig {
    pub fn parse_from_env(prefix: &str) -> DatabaseConfig {
        let db_prefix_s = format!("{}_DB", prefix);
        let db_prefix = db_prefix_s.as_str();
        let mut config = DatabaseConfig::default();

        let env_protocol = env::var(make_env_key(db_prefix, "PROTOCOL"));
        let env_host = env::var(make_env_key(db_prefix, "HOST"));
        let env_port = env::var(make_env_key(db_prefix, "PORT"));
        let env_user = env::var(make_env_key(db_prefix, "USER"));
        let env_pass = env::var(make_env_key(db_prefix, "PASS"));
        let env_database = env::var(make_env_key(db_prefix, "DATABASE"));

        let env_max_connections = env::var(make_env_key(db_prefix, "MAX_CONNECTIONS"));
        let env_min_connections = env::var(make_env_key(db_prefix, "MIN_CONNECTIONS"));
        let env_connect_timeout = env::var(make_env_key(db_prefix, "CONNECT_TIMEOUT"));
        let env_idle_timeout = env::var(make_env_key(db_prefix, "IDLE_TIMEOUT"));

        if let Ok(protocol) = env_protocol {
            config.protocol = protocol;
        }

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

        if let Ok(max_connections) = env_max_connections {
            config.max_connections = max_connections
                .parse::<i32>()
                .unwrap_or(config.max_connections);
        }

        if let Ok(min_connections) = env_min_connections {
            config.min_connections = min_connections
                .parse::<i32>()
                .unwrap_or(config.min_connections);
        }

        if let Ok(connect_timeout) = env_connect_timeout {
            config.connect_timeout = connect_timeout
                .parse::<i32>()
                .unwrap_or(config.connect_timeout);
        }

        if let Ok(idle_timeout) = env_idle_timeout {
            config.idle_timeout = idle_timeout.parse::<i32>().unwrap_or(config.idle_timeout);
        }

        return config;
    }

    pub fn layer(first: DatabaseConfig, second: DatabaseConfig) -> DatabaseConfig {
        let mut config = first.clone();
        let default = DatabaseConfig::default();

        if first.protocol == default.protocol {
            config.protocol = second.protocol;
        }

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

        if first.min_connections == default.min_connections {
            config.min_connections = second.min_connections;
        }

        if first.max_connections == default.max_connections {
            config.max_connections = second.max_connections;
        }

        if first.connect_timeout == default.connect_timeout {
            config.connect_timeout = second.connect_timeout;
        }

        if first.idle_timeout == default.idle_timeout {
            config.idle_timeout = second.idle_timeout;
        }

        return config;
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        return Self {
            protocol: "postgresql".to_string(),
            host: "127.0.0.1".to_string(),
            port: 5432,
            user: None,
            pass: None,
            database: "postgres".to_string(),

            min_connections: 64,
            max_connections: 1024,
            connect_timeout: 5,
            idle_timeout: 120,
        };
    }
}
