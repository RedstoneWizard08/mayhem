use mayhem_db::client::connector::{
    Authentication, ConnectionOptions, DatabaseProtocol, PasswordAuthentication,
};

pub mod login;
pub mod register;

pub fn prepare_connection() -> ConnectionOptions {
    return ConnectionOptions {
        protocol: DatabaseProtocol::PostgreSQL,
        auth: Authentication::Password(PasswordAuthentication {
            user: "mayhem".to_string(),
            pass: "mayhem".to_string(),
        }),
        host: "127.0.0.1".to_string(),
        port: 5432,
        database: "mayhem_dev".to_string(),
    };
}
