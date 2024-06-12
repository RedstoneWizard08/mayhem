#[derive(
    Debug,
    Clone,
    Default,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    DbEnum,
)]
#[ExistingTypePath = "crate::schema::sql_types::Channelkind"]
pub enum ChannelKind {
    #[default]
    Text,
    Voice,
    Announcement,
    Event,
}

#[derive(
    Debug,
    Clone,
    Default,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    DbEnum,
)]
#[ExistingTypePath = "crate::schema::sql_types::Theme"]
pub enum Theme {
    #[default]
    Dark,
    Light,
}
