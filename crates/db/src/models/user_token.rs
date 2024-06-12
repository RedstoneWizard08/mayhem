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
#[diesel(table_name = crate::schema::user_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
pub struct UserToken {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,

    pub user_id: i32,
    pub created: i64,
    pub last_used: Option<i64>,
    pub value: String,
}
