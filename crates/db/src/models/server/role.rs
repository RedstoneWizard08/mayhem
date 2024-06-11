use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity, Serialize, Deserialize)]
pub struct Entity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub server_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn, Serialize, Deserialize)]
pub enum Column {
    Id,
    Name,
    ServerId,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey, Serialize, Deserialize)]
pub enum PrimaryKey {
    Id,
}

#[derive(Copy, Clone, Debug, EnumIter, Serialize, Deserialize)]
pub enum Relation {
    Members,
    Server,
}

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        return "server_roles";
    }
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Name => ColumnType::Text.def(),
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

impl Related<super::member::Entity> for Entity {
    fn to() -> RelationDef {
        return super::member_role::Relation::Member.def();
    }

    fn via() -> Option<RelationDef> {
        return Some(super::member_role::Relation::Role.def().rev());
    }
}

impl Related<super::server::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Server.def();
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Members => Entity::has_many(super::member::Entity).into(),

            Self::Server => Entity::belongs_to(super::server::Entity)
                .from(Column::ServerId)
                .to(super::server::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
