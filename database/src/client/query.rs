use crate::{models::{user, server::{server, role, member, channel}, user_settings}, util::{CompleteServer, CompleteRole, CompleteMember, CompleteUser}};
use migration::DbErr;
use sea_orm::{DbConn, EntityTrait, ColumnTrait, QueryFilter, ModelTrait};
use std::sync::Arc;

#[derive(Clone)]
pub struct DatabaseQueryHelper {
    client: Arc<DbConn>,
}

unsafe impl Sync for DatabaseQueryHelper {}
unsafe impl Send for DatabaseQueryHelper {}

impl DatabaseQueryHelper {
    pub fn create(client: Arc<DbConn>) -> Self {
        return Self { client };
    }

    pub async fn find_user_by_id(&self, id: i32) -> Result<Option<user::Model>, DbErr> {
        return user::Entity::find_by_id(id).one(&self.client as &DbConn).await;
    }

    pub async fn find_user_by_name(&self, username: String) -> Result<Option<CompleteUser>, DbErr> {
        let user_result = user::Entity::find()
            .filter(user::Column::Username.eq(username.clone()))
            .one(&self.client as &DbConn)
            .await;
        
        let user_option: Option<user::Model>;

        match user_result {
            Ok(res) => user_option = res,
            Err(err) => return Err(err),
        };
        
        let user: user::Model;
        
        match user_option {
            Some(res) => user = res,
            None => return Ok(None),
        };

        let settings = user.find_related(user_settings::Entity).one(&self.client as &DbConn).await.unwrap().unwrap();

        let mut servers: Vec<CompleteServer> = Vec::new();
        let servers_query: Vec<server::Model>;

        let servers_result = user
            .find_related(server::Entity)
            .all(&self.client as &DbConn)
            .await;
        
        match servers_result {
            Ok(res) => servers_query = res,
            Err(err) => return Err(err),
        };

        for server in servers_query {
            let roles_found = server.clone().find_related(role::Entity).all(&self.client as &DbConn).await.unwrap();
            let members_found = server.clone().find_related(member::Entity).all(&self.client as &DbConn).await.unwrap();
            let channels = server.clone().find_related(channel::Entity).all(&self.client as &DbConn).await.unwrap();

            let mut roles: Vec<CompleteRole> = Vec::new();
            let mut members: Vec<CompleteMember> = Vec::new();

            for role in roles_found {
                let members = role
                    .find_related(member::Entity)
                    .all(&self.client as &DbConn)
                    .await
                    .unwrap()
                    .iter()
                    .map(|val| val.id)
                    .collect();

                roles.push(CompleteRole {
                    id: role.id,
                    name: role.name,
                    server_id: role.server_id,
                    member_ids: members,
                });
            }

            for member in members_found {
                let roles_local = member
                    .find_related(role::Entity)
                    .all(&self.client as &DbConn)
                    .await
                    .unwrap()
                    .iter()
                    .map(|val| val.id)
                    .collect();

                members.push(CompleteMember {
                    id: member.id,
                    name: member.name,
                    nick: member.nick,
                    server_id: member.server_id,
                    role_ids: roles_local,
                });
            }

            servers.push(CompleteServer {
                id: server.id,
                name: server.name,
                roles,
                members,
                channels,
            });
        }

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
