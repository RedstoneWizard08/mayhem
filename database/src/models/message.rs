use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity, Serialize, Deserialize)]
pub struct Entity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub user_id: i32,
    pub channel_id: i32,
    pub timestamp: String,
    pub content: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn, Serialize, Deserialize)]
pub enum Column {
    Id,
    UserId,
    ChannelId,
    Timestamp,
    Content,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey, Serialize, Deserialize)]
pub enum PrimaryKey {
    Id,
}

#[derive(Copy, Clone, Debug, EnumIter, Serialize, Deserialize)]
pub enum Relation {
    User,
    Channel,
}

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        return "messages";
    }
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::UserId => ColumnType::Integer.def(),
            Self::ChannelId => ColumnType::Integer.def(),
            Self::Timestamp => ColumnType::Text.def(),
            Self::Content => ColumnType::Text.def(),
        }
    }
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;

    fn auto_increment() -> bool {
        return true;
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::User.def();
    }
}

impl Related<super::server::channel::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Channel.def();
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::UserId)
                .to(super::user::Column::Id)
                .into(),

            Self::Channel => Entity::belongs_to(super::server::channel::Entity)
                .from(Column::ChannelId)
                .to(super::server::channel::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
