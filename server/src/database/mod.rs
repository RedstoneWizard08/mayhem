use mayhem_db::client::connector::{
    DatabaseConnectionOptions,
    DatabaseProtocol,
    UserAuthentication,
    DatabaseAuthentication,
};

pub mod login;
pub mod migrate;
pub mod register;

pub fn prepare_connection() -> DatabaseConnectionOptions {
    return DatabaseConnectionOptions {
        protocol: DatabaseProtocol::PostgreSQL,
        auth: DatabaseAuthentication::User(UserAuthentication {
            user: "postgres".to_string()
        }),
        host: "127.0.0.1".to_string(),
        port: 5432,
        database: "mayhem_dev".to_string()
    };
}
