use crate::models::Server;

#[derive(
    Debug,
    Clone,
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
#[diesel(table_name = crate::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Server))]
pub struct Role {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,

    pub server_id: i32,
    pub name: String,
    pub color: Option<String>,
    pub created: i64,
    pub updated: Option<i64>,

    // Permissions
    pub send_messages: bool,
    pub join_voice: bool,
    pub enable_camera: bool,
}

impl Default for Role {
    fn default() -> Self {
        Self {
            id: None,
            server_id: 0,
            name: String::new(),
            color: None,
            created: 0,
            updated: None,

            send_messages: true,
            join_voice: true,
            enable_camera: true,
        }
    }
}
