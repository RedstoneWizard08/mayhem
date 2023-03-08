use sea_orm::{ColumnTrait, DbConn, DbErr, EntityTrait, ModelTrait, QueryFilter};
use std::sync::Arc;

use super::{common::ConnectionHolder, server::ServerQueryHelper};
use crate::{
    models::{
        server::{channel, server},
        user, user_settings,
    },
    util::{CompleteMember, CompleteRole, CompleteServer, CompleteUser},
};

#[derive(Clone)]
pub struct UserQueryHelper {
    pub client: Arc<DbConn>,
    pub server: Arc<ServerQueryHelper>,
}

unsafe impl Sync for UserQueryHelper {}
unsafe impl Send for UserQueryHelper {}

impl ConnectionHolder for UserQueryHelper {
    fn get_connection(&self) -> &DbConn {
        return &self.client as &DbConn;
    }

    fn get_connection_ref(&self) -> Arc<DbConn> {
        return self.client.clone();
    }

    fn get_connection_raw(&self) -> *const DbConn {
        return Arc::into_raw(self.client.clone());
    }
}

impl UserQueryHelper {
    pub async fn find_user_servers(
        &self,
        user: &user::Model,
    ) -> Result<Vec<CompleteServer>, DbErr> {
        let mut servers: Vec<CompleteServer> = Vec::new();
        
        let servers_result = user
            .find_related(server::Entity)
            .all(&self.client as &DbConn)
            .await;

        let servers_query: Vec<server::Model> = match servers_result {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        for server in servers_query {
            let roles_found = self.server.find_server_roles(&server).await;
            let members_found = self.server.find_server_members(&server).await;
            let channels_found = self.server.find_server_channels(&server).await;

            let roles: Vec<CompleteRole> = match roles_found {
                Ok(val) => val,
                Err(err) => return Err(err),
            };

            let members: Vec<CompleteMember> = match members_found {
                Ok(val) => val,
                Err(err) => return Err(err),
            };

            let channels: Vec<channel::Model> = match channels_found {
                Ok(val) => val,
                Err(err) => return Err(err),
            };

            servers.push(CompleteServer {
                id: server.id,
                name: server.name,
                roles,
                members,
                channels,
            });
        }

        return Ok(servers);
    }

    pub async fn find_user_by_id(&self, id: i32) -> Result<Option<user::Model>, DbErr> {
        return user::Entity::find_by_id(id)
            .one(&self.client as &DbConn)
            .await;
    }

    pub async fn find_user_by_name(&self, username: String) -> Result<Option<CompleteUser>, DbErr> {
        let user_result = user::Entity::find()
            .filter(user::Column::Username.eq(username.clone()))
            .one(&self.client as &DbConn)
            .await;

        let user_option: Option<user::Model> = match user_result {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let user: user::Model = match user_option {
            Some(res) => res,
            None => return Ok(None),
        };

        let servers_result = self.find_user_servers(&user).await;

        let servers: Vec<CompleteServer> = match servers_result {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let settings = user
            .find_related(user_settings::Entity)
            .one(&self.client as &DbConn)
            .await
            .unwrap();

        return Ok(Some(CompleteUser {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            username: user.username,
            password: user.password,
            servers,
            settings,
        }));
    }
}
