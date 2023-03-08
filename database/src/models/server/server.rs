use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity, Serialize, Deserialize)]
pub struct Entity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn, Serialize, Deserialize)]
pub enum Column {
    Id,
    Name,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey, Serialize, Deserialize)]
pub enum PrimaryKey {
    Id,
}

#[derive(Copy, Clone, Debug, EnumIter, Serialize, Deserialize)]
pub enum Relation {
    Roles,
    Members,
    Channels,
}

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        return "servers";
    }
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Name => ColumnType::Text.def(),
        }
    }
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;

    fn auto_increment() -> bool {
        return true;
    }
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Roles.def();
    }
}

impl Related<super::member::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Members.def();
    }
}

impl Related<super::channel::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Channels.def();
    }
}

impl Related<super::super::user::Entity> for Entity {
    fn to() -> RelationDef {
        return super::super::user_server::Relation::User.def();
    }

    fn via() -> Option<RelationDef> {
        return Some(super::super::user_server::Relation::Server.def().rev());
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Roles => Entity::has_many(super::role::Entity).into(),
            Self::Members => Entity::has_many(super::member::Entity).into(),
            Self::Channels => Entity::has_many(super::channel::Entity).into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
