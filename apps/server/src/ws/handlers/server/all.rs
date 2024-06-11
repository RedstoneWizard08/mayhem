use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{stream::SplitSink, SinkExt};
use mayhem_db::{
    models::{EServer, EUser},
    sea_orm::{DatabaseConnection, EntityTrait, ModelTrait},
};
use tokio::sync::Mutex;

use crate::ws::handlers::{ActiveMessage, ActiveMessageAction};

use super::ServersData;

pub async fn on_get_servers(
    user_id: i32,
    db: &DatabaseConnection,
    wtx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
) {
    let user_res = EUser::find_by_id(user_id).one(db).await;

    if let Err(err) = &user_res {
        wtx.lock()
            .await
            .send(Message::Text(err.to_string()))
            .await
            .unwrap();

        return;
    }

    let user_opt = user_res.unwrap();

    if let Some(user) = user_opt {
        let server_res = user.find_related(EServer).all(db).await;

        if let Err(err) = &server_res {
            wtx.lock()
                .await
                .send(Message::Text(err.to_string()))
                .await
                .unwrap();

            return;
        }

        let servers = server_res.unwrap();

        let data_struct = ActiveMessage {
            action: ActiveMessageAction::GetServersForUser,
            data: ServersData { user_id, servers },
        };

        let data = serde_json::to_string(&data_struct).unwrap();

        wtx.lock().await.send(Message::Text(data)).await.unwrap();
    } else {
        wtx.lock()
            .await
            .send(Message::Text(
                "Could not get the user from the database!".to_string(),
            ))
            .await
            .unwrap();
    }
}
