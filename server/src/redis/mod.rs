pub mod sockets;

use redis::{Client, RedisError, aio::Connection};

pub async fn create_connection(host: String, port: i32) -> Result<Connection, RedisError> {
    let uri = format!("redis://{}:{}/", host, port);
    let client = Client::open(uri)?;
    
    return client.get_async_connection().await;
}
