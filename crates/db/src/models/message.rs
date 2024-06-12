use crate::models::{Channel, User};

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
#[diesel(table_name = crate::schema::messages)]
#[diesel(belongs_to(Channel))]
#[diesel(belongs_to(User, foreign_key = sender_id))]
pub struct Message {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,

    pub content: String,
    pub created: i64,
    pub updated: Option<i64>,
    pub edited: bool,
    pub to_friend: bool,
    pub sender_id: i32,
    pub channel_id: i32,
}
