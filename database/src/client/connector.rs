use rocket::serde::{Deserialize, Serialize};
use sea_orm::{Database, DatabaseConnection, DbErr};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum DatabaseProtocol {
    PostgreSQL,
    SQLite,
    MySQL,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum DatabaseAuthentication {
    Password(PasswordAuthentication),
    User(UserAuthentication),
    Anonymous,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
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
#[serde(crate = "rocket::serde")]
pub struct UserAuthentication {
    pub user: String,
}

impl UserAuthentication {
    pub fn get_auth_string(&self) -> String {
        return self.user.clone() + "@";
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DatabaseConnectionOptions {
    pub protocol: DatabaseProtocol,

    pub auth: DatabaseAuthentication,
    pub host: String,
    pub port: i32,
    pub database: String,
}

impl DatabaseConnectionOptions {
    pub fn get_protocol(&self) -> String {
        return (match self.protocol.clone() {
            DatabaseProtocol::PostgreSQL => "postgres://",
            DatabaseProtocol::MySQL => "mysql://",
            DatabaseProtocol::SQLite => "sqlite://",
        })
        .to_string();
    }

    pub fn get_connection_string(&self) -> String {
        let auth_str;

        match self.auth.clone() {
            DatabaseAuthentication::Password(val) => auth_str = val.get_auth_string(),
            DatabaseAuthentication::User(val) => auth_str = val.get_auth_string(),
            DatabaseAuthentication::Anonymous => auth_str = "".to_string(),
        };

        return self.get_protocol()
            + auth_str.as_str()
            + self.host.clone().as_str()
            + ":"
            + self.port.clone().to_string().as_str()
            + "/"
            + self.database.clone().to_string().as_str();
    }
}

pub async fn connect(opts: DatabaseConnectionOptions) -> Result<DatabaseConnection, DbErr> {
    return Database::connect(opts.get_connection_string()).await;
}
