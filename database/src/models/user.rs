use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity, Serialize, Deserialize)]
pub struct Entity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i32,

    pub first_name: String,
    pub last_name: String,

    #[sea_orm(column_type = "Text", unique, indexed)]
    pub email: String,

    #[sea_orm(column_type = "Text", unique, indexed)]
    pub username: String,
    pub password: String,
}

impl Model {
    pub fn from_active(active: ActiveModel) -> Model {
        return Model {
            id: active.id.unwrap(),
            first_name: active.first_name.unwrap(),
            last_name: active.last_name.unwrap(),
            email: active.email.unwrap(),
            username: active.username.unwrap(),
            password: active.password.unwrap(),
        };
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn, Serialize, Deserialize)]
pub enum Column {
    Id,
    FirstName,
    LastName,
    Email,
    Username,
    Password,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey, Serialize, Deserialize)]
pub enum PrimaryKey {
    Id,
}

#[derive(Copy, Clone, Debug, EnumIter, Serialize, Deserialize)]
pub enum Relation {
    Settings,
    Servers,
}

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        return "users";
    }
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::FirstName => ColumnType::Text.def(),
            Self::LastName => ColumnType::Text.def(),
            Self::Email => ColumnType::Text.def().unique(),
            Self::Username => ColumnType::Text.def().unique(),
            Self::Password => ColumnType::Text.def(),
        }
    }
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;

    fn auto_increment() -> bool {
        return true;
    }
}

impl Related<super::user_settings::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Settings.def();
    }
}

impl Related<super::server::server::Entity> for Entity {
    fn to() -> RelationDef {
        return super::user_server::Relation::Server.def();
    }

    fn via() -> Option<RelationDef> {
        return Some(super::user_server::Relation::User.def().rev());
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Settings => Entity::has_one(super::user_settings::Entity).into(),
            Self::Servers => Entity::has_many(super::server::server::Entity).into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
