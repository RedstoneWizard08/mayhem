pub mod finder;
pub mod channel;
pub mod message;
pub mod server;

use std::{sync::Arc, collections::HashMap};

use axum::{Error, extract::ws::Message};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc::UnboundedSender, Mutex};

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

#[derive(Debug, Clone)]
pub struct WsClient {
    pub client_id: String,
    pub sender: Option<UnboundedSender<Result<Message, Error>>>,
}

pub type WsClients = Arc<Mutex<HashMap<String, WsClient>>>;
