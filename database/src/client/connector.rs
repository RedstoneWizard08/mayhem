use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum DatabaseProtocol {
    PostgreSQL,
    SQLite,
    MySQL,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Authentication {
    Password(PasswordAuthentication),
    User(UserAuthentication),
    Anonymous,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordAuthentication {
    pub user: String,
    pub pass: String,
}

impl PasswordAuthentication {
    pub fn get_auth_string(&self) -> String {
        return self.user.clone() + ":" + self.pass.clone().as_str() + "@";
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserAuthentication {
    pub user: String,
}

impl UserAuthentication {
    pub fn get_auth_string(&self) -> String {
        return self.user.clone() + "@";
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectionOptions {
    pub protocol: DatabaseProtocol,

    pub auth: Authentication,
    pub host: String,
    pub port: i32,
    pub database: String,
}

impl ConnectionOptions {
    pub fn get_protocol(&self) -> String {
        return (match self.protocol {
            DatabaseProtocol::PostgreSQL => "postgres://",
            DatabaseProtocol::MySQL => "mysql://",
            DatabaseProtocol::SQLite => "sqlite://",
        })
        .to_string();
    }

    pub fn get_connection_string(&self) -> String {
        let auth_str = match self.auth.clone() {
            Authentication::Password(val) => val.get_auth_string(),
            Authentication::User(val) => val.get_auth_string(),
            Authentication::Anonymous => "".to_string(),
        };

        return self.get_protocol()
            + auth_str.as_str()
            + self.host.clone().as_str()
            + ":"
            + self.port.clone().to_string().as_str()
            + "/"
            + self.database.to_string().as_str();
    }
}

pub async fn connect(opts: ConnectionOptions) -> Result<DatabaseConnection, DbErr> {
    let mut conn = ConnectOptions::new(opts.get_connection_string());

    conn.sqlx_logging(false)
        .sqlx_logging_level(log::LevelFilter::Error);

    return Database::connect(conn).await;
}
