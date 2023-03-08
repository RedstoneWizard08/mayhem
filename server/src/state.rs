use crate::ws::ChatRoom;
use mayhem_db::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

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
