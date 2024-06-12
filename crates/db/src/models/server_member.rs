use crate::models::{Server, User};

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
#[diesel(table_name = crate::schema::server_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Server))]
#[diesel(primary_key(user_id, server_id))]
pub struct ServerMember {
    pub user_id: i32,
    pub server_id: i32,
    pub nickname: Option<String>,
}
