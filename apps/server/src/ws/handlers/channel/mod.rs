pub mod all;
pub mod create;
pub mod finder;
pub mod get;

pub use all::on_get_channels;
pub use create::on_create_channel;
pub use finder::find_channel_handler;
pub use get::on_get_channel;

use mayhem_db::models::server::channel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelCreateData {
    pub name: String,
    pub server_id: i32,
    pub channel_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelsData {
    pub server_id: i32,
    pub channels: Vec<channel::Model>,
}
