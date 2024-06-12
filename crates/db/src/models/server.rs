use crate::models::User;

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
#[diesel(table_name = crate::schema::servers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User, foreign_key = owner_id))]
pub struct Server {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,

    pub name: String,
    pub icon_url: Option<String>,
    pub created: i64,
    pub updated: Option<i64>,
    pub owner_id: i32,
}
