pub mod all;
pub mod create;
pub mod delete;
pub mod finder;
pub mod get;
pub mod join;
pub mod leave;

pub use all::on_get_servers;
pub use create::on_create_server;
pub use delete::on_delete_server;
pub use finder::find_server_handler;
pub use get::on_get_server;
pub use join::on_join_server;
pub use leave::on_leave_server;

use mayhem_db::models::server::server;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerCreateData {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServersData {
    pub user_id: i32,
    pub servers: Vec<server::Model>,
}
