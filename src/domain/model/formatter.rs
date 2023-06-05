pub mod formatter {
    pub mod datetime {
        use chrono::{DateTime, Utc, TimeZone};
        use serde::{self, Deserialize, Serializer, Deserializer};

        const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
        pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer {
                serializer.serialize_str(&format_date(*date))
            }
        pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
            where D: Deserializer<'de> {
                let s = String::deserialize(deserializer)?;
                Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
            }
        pub fn format_date(date: DateTime<Utc>) -> String {
            format!("{}", date.format(FORMAT))
        }
    }
}