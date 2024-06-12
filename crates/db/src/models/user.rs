use chrono::{DateTime, Utc};

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
)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,

    pub username: String,
    pub email: String,

    #[serde(skip)]
    pub password: String,
    pub birthday: i64,
    pub created: i64,
    pub updated: Option<i64>,
}

impl User {
    pub fn birthday(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp_millis(self.birthday)
    }
}
