use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Entity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub server_id: i32,
    pub channel_type: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Column {
    Id,
    Name,
    ServerId,
    ChannelType,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum PrimaryKey {
    Id,
}

#[derive(Copy, Clone, Debug, EnumIter, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Relation {
    Server,
    Message,
}

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        return "server_channels";
    }
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Name => ColumnType::Text.def(),
            Self::ServerId => ColumnType::Integer.def(),
            Self::ChannelType => ColumnType::Text.def(),
        }
    }
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;

    fn auto_increment() -> bool {
        return true;
    }
}

impl Related<super::server::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Server.def();
    }
}

impl Related<super::super::message::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Message.def();
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Server => Entity::belongs_to(super::server::Entity)
                .from(Column::ServerId)
                .to(super::server::Column::Id)
                .into(),

            Self::Message => Entity::has_many(super::super::message::Entity).into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
