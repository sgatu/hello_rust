use super::formatter::formatter;
use crate::domain::model::User;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[readonly::make]
pub struct UserResponse {
    name: String,
    email: String,
    #[serde(with = "formatter::string")]
    id: u64,
    #[serde(with = "formatter::datetime")]
    created: NaiveDateTime,
}
impl UserResponse {
    pub fn new(user: User) -> Self {
        Self {
            name: user.name,
            email: user.email,
            id: user.id,
            created: user.created,
        }
    }
}
