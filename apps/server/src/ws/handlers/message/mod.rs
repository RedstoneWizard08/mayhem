pub mod finder;
pub mod get;
pub mod send;

pub use finder::find_message_handler;
pub use get::on_get_all_messages;
pub use send::on_message_send;

use mayhem_db::models::message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessageIn {
    pub content: String,
    pub timestamp: String,
    pub sender: i32,
    pub channel: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessagesData {
    pub channel_id: i32,
    pub messages: Vec<message::Model>,
}
