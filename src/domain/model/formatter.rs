pub mod formatter {
    pub mod datetime {
        use chrono::NaiveDateTime;
        use serde::{self, Deserialize, Serializer, Deserializer};

        pub const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
        pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer {
                serializer.serialize_str(&format_date(*date))
            }
        pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
            where D: Deserializer<'de> {
                let s = String::deserialize(deserializer)?;
                NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
            }
        pub fn format_date(date: NaiveDateTime) -> String {
            format!("{}", date.format(FORMAT))
        }
    }
}