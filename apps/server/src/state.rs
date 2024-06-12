use crate::{config::AppConfig, ws::ChatRoom};
use hyper::{
    client::{Client as RequestClient, HttpConnector},
    Body,
};
use mayhem_db::diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<AsyncPgConnection>,
    pub rooms: Arc<Mutex<Vec<ChatRoom>>>,
    pub config: AppConfig,
    pub request_client: RequestClient<HttpConnector, Body>,
}

impl AppState {
    pub fn new(pool: Pool<AsyncPgConnection>, config: AppConfig) -> Self {
        return Self {
            pool,
            rooms: Arc::new(Mutex::new(Vec::new())),
            config,
            request_client: RequestClient::new(),
        };
    }
}
