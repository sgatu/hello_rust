use super::formatter::formatter;
use crate::domain::model::User;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[readonly::make]
#[serde(crate = "rocket::serde")]
pub struct UserResponse {
    name: String,
    email: String,
    #[serde(with = "formatter::string")]
    id: u64,
    #[serde(with = "formatter::datetime")]
    created: NaiveDateTime,
}
impl UserResponse {
    pub fn new(user: &User) -> Self {
        Self {
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            id: user.id,
            created: user.created,
        }
    }
}
