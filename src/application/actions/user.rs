use rocket::{
    http::Status,
    serde::{
        json::{serde_json::json, Json, Value},
        Deserialize, Serialize,
    },
    State,
};

use crate::{
    application::responders::ApiError,
    domain::{
        model::{SessionData, User, UserError},
        repository::{SessionError, SessionRepository, UserRegistrationError},
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
pub fn get_me(session_data: SessionData) -> Json<User> {
    Json(session_data.user.to_owned())
}
#[get("/greet")]
pub fn greet(session_data: SessionData) -> Value {
    json!({ "message": format!("Hello {}", session_data.user.name) })
}

#[post["/register", data = "<registration>"]]
pub fn register(
    registration: Json<Registration>,
    session_repository: &State<Box<dyn SessionRepository>>,
) -> Result<Json<User>, ApiError> {
    let user = User::new(
        &registration.name,
        &registration.email,
        &registration.password,
    );
    match user {
        Ok(u) => Ok(Json(session_repository.register_user(u).map_err(
            |e| match e {
                UserRegistrationError::Existing => ApiError {
                    msg: "User with same email addresss already exists".to_string(),
                    status: Status::BadRequest,
                },
                _ => ApiError {
                    msg: "Something unexpected happened".to_string(),
                    status: Status::InternalServerError,
                },
            },
        )?)),
        Err(UserError::InvalidEmail) => Err(ApiError {
            msg: "Invalid email specified".to_string(),
            status: Status::BadRequest,
        }),
        Err(UserError::InvalidPassword) => Err(ApiError {
            msg: "Password is not secure enough. Try a harder one.".to_string(),
            status: Status::BadRequest,
        }),
    }
}

#[post("/", data = "<request_data>")]
pub fn login(
    request_data: Json<Credentials>,
    session_repository: &State<Box<dyn SessionRepository>>,
) -> Result<Json<SessionData>, Status> {
    let session_data = session_repository
        .create_session(&request_data.email, &request_data.password)
        .map_err(|e| match e {
            SessionError::Invalid => Status::Unauthorized,
            _ => Status::InternalServerError,
        })?;
    Ok(Json(session_data))
}

#[delete("/")]
pub fn revoke_token(
    mut session: SessionData,
    session_repository: &State<Box<dyn SessionRepository>>,
) -> Result<Value, Status> {
    session = session.revoke();
    session_repository
        .save(session)
        .map_err(|_e| Status::InternalServerError)?;
    Ok(json!({"status":"ok"}))
}
