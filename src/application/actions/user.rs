use rocket::{
    http::Status,
    serde::{
        json::{serde_json::json, Json, Value},
        Deserialize, Serialize,
    },
    State,
};

use crate::{
    application::responders::{ApiError, SessionReponse, UserResponse},
    domain::{
        model::{SessionData, User},
        repository::{SessionError, SessionRepository},
    },
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Credentials {
    email: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Registration {
    email: String,
    password: String,
    name: String,
}

#[get("/me")]
pub fn get_me(session_data: SessionData) -> Json<UserResponse> {
    Json(UserResponse::new(&session_data.user))
}
#[get("/greet")]
pub fn greet(session_data: SessionData) -> Value {
    json!({ "message": format!("Hello {}", session_data.user.name) })
}

#[post["/register", data = "<registration>"]]
pub fn register(
    registration: Json<Registration>,
    session_repository: &State<Box<dyn SessionRepository>>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = User::new(
        &registration.name,
        &registration.email,
        &registration.password,
    )?;
    let user = session_repository.register_user(user)?;
    Ok(Json(UserResponse::new(&user)))
}

#[post("/", data = "<request_data>")]
pub fn login(
    request_data: Json<Credentials>,
    session_repository: &State<Box<dyn SessionRepository>>,
) -> Result<Json<SessionReponse>, Status> {
    let session_data = session_repository
        .create_session(&request_data.email, &request_data.password)
        .map_err(|e| match e {
            SessionError::Invalid => Status::Unauthorized,
            _ => Status::InternalServerError,
        })?;
    Ok(Json(SessionReponse::new(&session_data)))
}

#[delete("/")]
pub fn revoke_token(
    session: SessionData,
    session_repository: &State<Box<dyn SessionRepository>>,
) -> Result<Value, Status> {
    let updated_session = session.revoke();
    session_repository
        .save(updated_session)
        .map_err(|_e| Status::InternalServerError)?;
    Ok(json!({"status":"ok"}))
}
