use chrono::{DateTime, Datelike, Timelike, Utc};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Birthday {
    pub month: u32,
    pub day: u32,
    pub year: i32,
}

impl Birthday {
    pub fn as_timestamp(&self) -> Option<DateTime<Utc>> {
        DateTime::<Utc>::default()
            .with_month(self.month)?
            .with_day(self.day)?
            .with_year(self.year)?
            .with_hour(0)?
            .with_minute(0)?
            .with_second(0)?
            .with_nanosecond(0)
    }
}
