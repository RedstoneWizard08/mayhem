use std::sync::Arc;
use tokio::sync::Mutex;
use mayhem_db::Client;
use crate::ws::ChatRoom;

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<Client>,
    pub rooms: Arc<Mutex<Vec<ChatRoom>>>,
}

impl AppState {
    pub fn new(client: Arc<Client>) -> Self {
        return Self {
            client,
            rooms: Arc::new(Mutex::new(Vec::new())),
        };
    }
}
