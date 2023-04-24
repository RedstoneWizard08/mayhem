use std::collections::HashSet;

use redis::{aio::Connection, AsyncCommands, RedisError};

pub async fn get_sockets_in_channel(conn: &mut Connection, channel_id: i32) -> Result<Vec<String>, RedisError> {
    let channel_key = format!("channel:{}:sockets", channel_id);
    let members: HashSet<String> = conn.smembers(channel_key).await?;

    return Ok(members.into_iter().collect::<Vec<String>>());
}

pub async fn add_socket_to_channel(conn: &mut Connection, channel_id: i32, socket_id: String) -> Result<(), RedisError> {
    let channel_key = format!("channel:{}:sockets", channel_id);
    conn.sadd(channel_key, socket_id).await?;

    return Ok(());
}

pub async fn remove_socket_from_channel(conn: &mut Connection, channel_id: i32, socket_id: String) -> Result<(), RedisError> {
    let channel_key = format!("channel:{}:sockets", channel_id);
    conn.srem(channel_key, socket_id).await?;

    return Ok(());
}
