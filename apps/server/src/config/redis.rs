use std::env;

use serde::{Deserialize, Serialize};

use super::env::make_env_key;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RedisConfig {
    pub host: String,
    pub port: i32,
}

impl RedisConfig {
    pub fn parse_from_env(prefix: &str) -> RedisConfig {
        let db_prefix_s = format!("{}_REDIS", prefix);
        let db_prefix = db_prefix_s.as_str();
        let mut config = RedisConfig::default();

        let env_host = env::var(make_env_key(db_prefix, "HOST"));
        let env_port = env::var(make_env_key(db_prefix, "PORT"));

        if let Ok(host) = env_host {
            config.host = host;
        }

        if let Ok(port) = env_port {
            config.port = port.parse::<i32>().unwrap_or(config.port);
        }

        return config;
    }

    pub fn layer(first: RedisConfig, second: RedisConfig) -> RedisConfig {
        let mut config = first.clone();
        let default = RedisConfig::default();

        if first.host == default.host {
            config.host = second.host;
        }

        if first.port == default.port {
            config.port = second.port;
        }

        return config;
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        return Self {
            host: "127.0.0.1".to_string(),
            port: 5432,
        };
    }
}
