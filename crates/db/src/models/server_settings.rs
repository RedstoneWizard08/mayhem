use crate::models::Server;

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
#[diesel(table_name = crate::schema::server_settings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Server))]
pub struct ServerSettings {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,

    pub server_id: i32,
    pub created: i64,
    pub updated: Option<i64>,

    // Settings
    pub is_public: bool,
}
