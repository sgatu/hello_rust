use super::user::User;
use chrono::{Duration, NaiveDateTime, Utc};

#[derive(Eq, PartialEq)]
pub struct SessionData {
    pub created: NaiveDateTime,
    pub expire: NaiveDateTime,
    pub token: String,
    pub user: User,
}
pub mod session_config {
    pub const SESSION_TOKEN_LENGTH: i32 = 24;
}
impl SessionData {
    pub fn from(token: String, user: User, expire: NaiveDateTime, created: NaiveDateTime) -> Self {
        SessionData {
            created: created,
            expire: expire,
            token: token,
            user: user,
        }
    }
    fn generate_token(length: i32) -> String {
        (0..length / 2)
            .map(|_| format!("{:02x}", rand::random::<u8>()))
            .collect()
    }
    pub fn new(user: User, expire_diff: i64) -> Self {
        let now = Utc::now();
        let created = now.naive_utc();
        let expire = now
            .checked_add_signed(Duration::seconds(expire_diff))
            .unwrap()
            .naive_utc();
        SessionData {
            created: created,
            expire: expire,
            token: Self::generate_token(session_config::SESSION_TOKEN_LENGTH),
            user: user,
        }
    }
    pub fn revoke(mut self) -> Self {
        self.expire = Utc::now().naive_utc();
        self
    }
}
