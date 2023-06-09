use crate::domain::model::{SessionData, User};
use std::result;
#[derive(Debug)]
pub enum SessionError {
    Missing,
    Expired,
    Error,
    Invalid,
}
pub enum UserRegistrationError {
    Existing,
    Error,
}
pub type ResultSession<T, E = SessionError> = result::Result<T, E>;
pub type ResultUser<T, E = UserRegistrationError> = result::Result<T, E>;
pub trait SessionRepository: Send + Sync {
    fn get_session(&self, token: &str) -> ResultSession<SessionData>;
    fn create_session(&self, email: &str, password: &str) -> ResultSession<SessionData>;
    fn save(&self, session: SessionData) -> ResultSession<SessionData>;
    fn register_user(&self, user: User) -> ResultUser<User>;
}
