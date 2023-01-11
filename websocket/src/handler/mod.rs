use serde::{Deserialize, Serialize};

pub mod channel;
pub mod message;
pub mod server;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum ActiveMessageAction {
    SendMessage,
    RecieveMessage,
    GetMessagesForChannel,

    CreateChannel,
    GetChannelInfo,
    GetChannelsInServer,

    CreateServer,
    GetServerInfo,
    JoinServer,
    LeaveServer,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActiveMessage<T> {
    pub action: ActiveMessageAction,
    pub data: T,
}
