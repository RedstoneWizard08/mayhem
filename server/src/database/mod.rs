use mayhem_db::client::connector::{
    Authentication, ConnectionOptions, DatabaseProtocol, PasswordAuthentication, UserAuthentication,
};

use crate::config::{AppConfig, DatabaseConfig};

pub mod login;
pub mod register;

pub fn proto_from_string(s: String) -> DatabaseProtocol {
    if s == "postgres" || s == "postgresql" {
        return DatabaseProtocol::PostgreSQL;
    }

    if s == "mysql" {
        return DatabaseProtocol::MySQL;
    }

    if s == "sqlite" || s == "sqlite3" {
        return DatabaseProtocol::SQLite;
    }

    panic!("Unknown protocol!");
}

pub fn make_auth(config: DatabaseConfig) -> Authentication {
    if config.user.is_some() {
        if config.pass.is_none() {
            return Authentication::User(UserAuthentication {
                user: config.user.unwrap(),
            });
        }

        return Authentication::Password(PasswordAuthentication {
            user: config.user.unwrap(),
            pass: config.pass.unwrap(),
        });
    }

    return Authentication::Anonymous;
}

pub fn prepare_connection(config: AppConfig) -> ConnectionOptions {
    return ConnectionOptions {
        protocol: proto_from_string(config.database.clone().protocol),
        auth: make_auth(config.database.clone()),
        host: config.database.clone().host,
        port: config.database.port,
        database: config.database.database,
    };
}
