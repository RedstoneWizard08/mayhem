pub mod sockets;

use redis::{aio::Connection, Client, RedisError};

pub async fn create_connection(host: String, port: i32) -> Result<Connection, RedisError> {
    let uri = format!("redis://{}:{}/", host, port);
    let client = Client::open(uri)?;

    return client.get_async_connection().await;
}
