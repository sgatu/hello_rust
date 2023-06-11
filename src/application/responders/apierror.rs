use rocket::{
    http::Status,
    response::{self, Responder, Response},
    serde::json::serde_json::json,
};

use crate::domain::{model::UserError, repository::UserRegistrationError};

pub struct ApiError {
    pub msg: String,
    pub status: Status,
}
impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> response::Result<'static> {
        let body = json!({
            "error": self.status.reason_lossy(),
            "message": self.msg,
            "code": self.status.code
        });
        Ok(Response::build_from(body.respond_to(request)?)
            .status(self.status)
            .finalize())
    }
}
impl From<UserError> for ApiError {
    fn from(error: UserError) -> Self {
        match error {
            UserError::InvalidEmail => ApiError {
                msg: "Invalid email specified".to_string(),
                status: Status::BadRequest,
            },
            UserError::InvalidPassword => ApiError {
                msg: "Password is not secure enough. Try a harder one.".to_string(),
                status: Status::BadRequest,
            },
        }
    }
}
impl From<UserRegistrationError> for ApiError {
    fn from(error: UserRegistrationError) -> Self {
        match error {
            UserRegistrationError::Existing => ApiError {
                msg: "User with same email addresss already exists".to_string(),
                status: Status::BadRequest,
            },
            _ => ApiError {
                msg: "Something unexpected happened".to_string(),
                status: Status::InternalServerError,
            },
        }
    }
}
