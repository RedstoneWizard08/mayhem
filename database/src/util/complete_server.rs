use crate::models::server::channel::Model as Channel;
use rocket::serde::{Deserialize, Serialize};

use super::{CompleteMember, CompleteRole};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CompleteServer {
    pub id: i32,
    pub name: String,
    pub roles: Vec<CompleteRole>,
    pub members: Vec<CompleteMember>,
    pub channels: Vec<Channel>,
}
