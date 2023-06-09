use super::user::User;
use chrono::NaiveDateTime;

pub struct SessionData {
    pub created: NaiveDateTime,
    pub expire: NaiveDateTime,
    pub token: String,
    pub user: User
}

