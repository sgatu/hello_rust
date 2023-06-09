use std::io::Cursor;

use rocket::{
    http::Status,
    response::{self, Responder, Response},
    serde::json::serde_json::json,
};

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
