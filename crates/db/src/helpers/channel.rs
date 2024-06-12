use chrono::Utc;
use diesel::{insert_into, SelectableHelper};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use crate::{
    error::ValidationError,
    models::{Channel, Message, User},
    schema::messages,
    Result,
};

impl Channel {
    pub async fn send_message(
        &self,
        user: &User,
        content: impl AsRef<str>,
        pool: &Pool<AsyncPgConnection>,
    ) -> Result<Message> {
        let has_perms = self
            .get_server(pool)
            .await?
            .get_roles_for_user(user, pool)
            .await?
            .iter()
            .any(|role| role.send_messages);

        if !has_perms {
            return Err(ValidationError::Permissions.into());
        }

        let msg = Message {
            channel_id: self.id.unwrap(),
            content: content.as_ref().to_string(),
            created: Utc::now().timestamp_millis(),
            sender_id: user.id.unwrap(),
            ..Default::default()
        };

        Ok(insert_into(messages::table)
            .values(msg)
            .returning(Message::as_returning())
            .get_result(&mut pool.get().await?)
            .await?)
    }
}
