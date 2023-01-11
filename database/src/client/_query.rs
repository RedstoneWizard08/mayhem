use crate::{
    models::{
        server::{channel, member, role, server},
        user, user_settings,
    },
    util::{CompleteMember, CompleteRole, CompleteServer, CompleteUser},
};
use migration::DbErr;
use sea_orm::{ColumnTrait, DbConn, EntityTrait, ModelTrait, QueryFilter};
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
}
