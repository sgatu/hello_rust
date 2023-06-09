use crate::{
    domain::{
        model::SessionData,
        repository::{SessionError, SessionRepository},
    },
    infrastructure::repository::MysqlSessionRepository,
};
use chrono::Utc;
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
    State,
};

/*#[derive(Debug)]
pub enum SessionError {
    Invalid,
    Missing
}*/
#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionData {
    type Error = SessionError;
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");
        let session_repository = request
            .guard::<&State<Box<dyn SessionRepository>>>()
            .await
            .unwrap()
            .inner();
        match token {
            Some(t) => {
                let session = session_repository.get_session(t);

                match session {
                    Ok(session)
                        if session.expire.timestamp() > Utc::now().naive_utc().timestamp() =>
                    {
                        Outcome::Success(session)
                    }
                    Err(_err) => Outcome::Failure((Status::Unauthorized, SessionError::Invalid)),
                    _ => Outcome::Failure((Status::Unauthorized, SessionError::Expired)),
                }
            }
            _ => Outcome::Failure((Status::Unauthorized, SessionError::Missing)),
        }
    }
}
