#[cfg(test)]
mod test_batch {
    use chrono::Utc;

    use crate::domain::model::User;
    fn get_valid_user() -> User {
        User::new("Test user", "test@email.com", "v$lidP4ssword").unwrap()
    }

    #[test]
    fn test_session_creation() {
        use crate::domain::model::{session_config, SessionData};
        let time_diff = 600;
        let session = SessionData::new(get_valid_user(), time_diff);
        let diff = session.expire - session.created;
        assert!(
            diff.num_seconds() == time_diff,
            "Session expiration should be {} seconds into the future but is {} seconds.",
            time_diff,
            diff.num_seconds()
        );
        assert!(
            session.token.len() as i32 == session_config::SESSION_TOKEN_LENGTH,
            "Invalid session token length"
        )
    }
    #[test]
    fn test_session_revoke() {
        use crate::domain::model::SessionData;
        let mut session = SessionData::new(get_valid_user(), 600);
        session = session.revoke();
        assert!(
            (Utc::now().naive_utc() - session.expire).num_seconds() >= 0,
            "Session revocation was unsuccesful"
        )
    }
}
