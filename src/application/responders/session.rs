use super::formatter::formatter;
use crate::domain::model::SessionData;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[readonly::make]
#[serde(crate = "rocket::serde")]
pub struct SessionReponse {
    token: String,
    #[serde(with = "formatter::datetime")]
    expire: NaiveDateTime,
    #[serde(with = "formatter::datetime")]
    created: NaiveDateTime,
}
impl SessionReponse {
    pub fn new(session_data: &SessionData) -> Self {
        Self {
            token: session_data.token.to_owned(),
            expire: session_data.expire,
            created: session_data.created,
        }
    }
}
