use tokio::sync::broadcast::{channel, Sender};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Room {
    pub id: String,
    pub sender: Sender<String>,
}

impl Room {
    pub fn new() -> Self {
        let (tx, _) = channel::<String>(1);

        return Self {
            id: Uuid::new_v4().to_string(),
            sender: tx,
        };
    }
}

impl Default for Room {
    fn default() -> Self {
        return Self::new();
    }
}
