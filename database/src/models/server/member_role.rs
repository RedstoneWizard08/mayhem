use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Entity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Model {
    pub member_id: i32,
    pub role_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Column {
    MemberId,
    RoleId,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum PrimaryKey {
    MemberId,
    RoleId,
}

#[derive(Copy, Clone, Debug, EnumIter, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Relation {
    Member,
    Role,
}

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        return "member_roles";
    }
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::MemberId => ColumnType::Integer.def(),
            Self::RoleId => ColumnType::Integer.def(),
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
        return Relation::Member.def();
    }
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Role.def();
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Member => Entity::belongs_to(super::member::Entity)
                .from(Column::MemberId)
                .to(super::member::Column::Id)
                .into(),

            Self::Role => Entity::belongs_to(super::role::Entity)
                .from(Column::RoleId)
                .to(super::role::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
