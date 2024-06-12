use crate::models::{Role, User};

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
#[diesel(table_name = crate::schema::member_roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(primary_key(user_id, role_id))]
pub struct MemberRole {
    pub user_id: i32,
    pub role_id: i32,
}
