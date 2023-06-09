
use std::result;
use crate::domain::model::SessionData;

pub enum SessionError {
    Missing,
    Expired,
    Error,
    Invalid
}
pub type Result<T, E = SessionError> = result::Result<T, E>;
pub trait SessionRepository {
    fn get_session(&self, token: &str) -> Result<SessionData>;
}