use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Entity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Model {
    pub user_id: i32,
    pub server_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Column {
    UserId,
    ServerId,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum PrimaryKey {
    UserId,
    ServerId,
}

#[derive(Copy, Clone, Debug, EnumIter, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Relation {
    User,
    Server,
}

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        return "user_servers";
    }
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::UserId => ColumnType::Integer.def(),
            Self::ServerId => ColumnType::Integer.def(),
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

impl Related<super::server::server::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Server.def();
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::UserId)
                .to(super::user::Column::Id)
                .into(),

            Self::Server => Entity::belongs_to(super::server::server::Entity)
                .from(Column::ServerId)
                .to(super::server::server::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
