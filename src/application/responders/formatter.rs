pub mod formatter {
    pub mod datetime {
        use chrono::NaiveDateTime;
        use serde::{self, Deserialize, Deserializer, Serializer};

        pub const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
        pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&format_date(*date))
        }
        pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
        }
        pub fn format_date(date: NaiveDateTime) -> String {
            format!("{}", date.format(FORMAT))
        }
    }
    pub mod string {
        use std::fmt::Display;
        use std::str::FromStr;

        use serde::{de, Deserialize, Deserializer, Serializer};

        pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            T: Display,
            S: Serializer,
        {
            serializer.collect_str(value)
        }

        pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
        where
            T: FromStr,
            T::Err: Display,
            D: Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(de::Error::custom)
        }
    }
}
