use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompleteMember {
    pub id: i32,
    pub name: String,
    pub nick: String,
    pub role_ids: Vec<i32>,
    pub server_id: i32,
}
