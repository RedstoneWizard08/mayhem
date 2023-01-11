use sea_orm::{DbConn, DbErr, ModelTrait};
use std::sync::Arc;

use super::common::ConnectionHolder;
use crate::{
    models::server::{channel, member, role, server},
    util::{CompleteMember, CompleteRole},
};

#[derive(Clone)]
pub struct ServerQueryHelper {
    pub client: Arc<DbConn>,
}

unsafe impl Sync for ServerQueryHelper {}
unsafe impl Send for ServerQueryHelper {}

impl ConnectionHolder for ServerQueryHelper {
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

impl ServerQueryHelper {
    pub async fn find_server_roles(
        &self,
        server: &server::Model,
    ) -> Result<Vec<CompleteRole>, DbErr> {
        let mut roles: Vec<CompleteRole> = Vec::new();
        let ok_roles: Vec<role::Model>;

        let found_roles = server
            .clone()
            .find_related(role::Entity)
            .all(&self.client as &DbConn)
            .await;

        match found_roles {
            Ok(res) => ok_roles = res,
            Err(err) => return Err(err),
        };

        for role in ok_roles {
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

        return Ok(roles);
    }

    pub async fn find_server_members(
        &self,
        server: &server::Model,
    ) -> Result<Vec<CompleteMember>, DbErr> {
        let mut members: Vec<CompleteMember> = Vec::new();
        let ok_members: Vec<member::Model>;

        let members_found = server
            .clone()
            .find_related(member::Entity)
            .all(&self.client as &DbConn)
            .await;

        match members_found {
            Ok(res) => ok_members = res,
            Err(err) => return Err(err),
        };

        for member in ok_members {
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

        return Ok(members);
    }

    pub async fn find_server_channels(
        &self,
        server: &server::Model,
    ) -> Result<Vec<channel::Model>, DbErr> {
        let channels = server
            .clone()
            .find_related(channel::Entity)
            .all(&self.client as &DbConn)
            .await;

        return channels;
    }
}
