use crate::{
    models::{ChannelCategory, Server},
    types::ChannelKind,
};

#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Queryable,
    Identifiable,
    Selectable,
    Insertable,
    Associations,
)]
#[diesel(table_name = crate::schema::channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Server))]
#[diesel(belongs_to(ChannelCategory, foreign_key = category_id))]
pub struct Channel {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,

    pub server_id: i32,
    pub category_id: i32,
    pub name: String,
    pub kind: ChannelKind,
    pub created: i64,
    pub updated: Option<i64>,
}
