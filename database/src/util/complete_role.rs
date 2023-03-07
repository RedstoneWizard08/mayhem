use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompleteRole {
    pub id: i32,
    pub name: String,
    pub server_id: i32,
    pub member_ids: Vec<i32>,
}
