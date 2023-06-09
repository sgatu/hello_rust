
use crate::{domain::{model::{SessionData, User}, repository::SessionRepository}, infrastructure::repository::MysqlSessionRepository};
use rocket::{request::{self, Request, FromRequest, Outcome}, http::Status, State};

#[derive(Debug)]
pub enum SessionError {
    Invalid,
    Missing
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionData {
        type Error = SessionError;
        async fn from_request(request: &'r Request<'_>) ->  request::Outcome<Self, Self::Error> {
            let token = request.headers().get_one("Authorization");
            let sessionRepository = request.guard::<&State<MysqlSessionRepository>>().await.unwrap().inner();
            match token {
                Some(t) =>{
                    let session = sessionRepository.get_session(t);
                    match session {
                        Ok(session) => Outcome::Success(session),
                        _ => Outcome::Failure((Status::Unauthorized, SessionError::Invalid))
                    }
                },
                _ => Outcome::Failure((Status::Unauthorized, SessionError::Missing))
            }
        }
}