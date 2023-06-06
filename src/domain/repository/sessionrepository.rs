
pub trait SessionRepository {
    fn get_session(&self, token: &str) -> SessionState;
}