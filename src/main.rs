#[macro_use] extern crate rocket;
use mysql::Pool;
use rocket::{serde::json::{Json, Value, serde_json::json}, http::Status, Request};
use dotenvy::{self, dotenv};
use std::env;

mod domain;
mod application;
mod infrastructure;
use domain::model::{User, SessionData};
#[get("/")]
async fn index<'a>() -> Json<String> {
    Json("Hello world!".to_string())
}
#[get("/user")]
fn get_user(session_data: SessionData) -> Json<User> {
    Json(session_data.user)
}
#[catch(default)]
fn err_handler(status: Status, _req: &Request)-> Value {
    json!({
        "error": status.code,
        "message": status.reason()
    })
}
#[launch]
fn rocket() -> _ {
    dotenv().expect("Missing .env file.");
    let pool: Pool = Pool::new(std::env::var("MYSQL_URL").unwrap().as_str()).unwrap();
    let mut rocket = rocket::build()
    .mount("/", routes![index, get_user])
    .register("/", catchers![err_handler]);
    rocket = infrastructure::repository::manage(rocket, pool);
    rocket
}