use super::user::User;
use chrono::{DateTime, Utc};

pub struct SessionData {
    pub created: DateTime<Utc>,
    pub expire: DateTime<Utc>,
    pub token: String,
    pub user: User
}

pub enum SessionState {
    Connected(SessionData),
    NotConnected
}