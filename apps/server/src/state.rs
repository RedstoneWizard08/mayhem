use crate::{config::AppConfig, ws::ChatRoom};
use hyper::{
    client::{Client as RequestClient, HttpConnector},
    Body,
};
use mayhem_db::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<Client>,
    pub rooms: Arc<Mutex<Vec<ChatRoom>>>,
    pub config: AppConfig,
    pub request_client: RequestClient<HttpConnector, Body>,
}

impl AppState {
    pub fn new(client: Arc<Client>, config: AppConfig) -> Self {
        return Self {
            client,
            rooms: Arc::new(Mutex::new(Vec::new())),
            config,
            request_client: RequestClient::new(),
        };
    }
}
