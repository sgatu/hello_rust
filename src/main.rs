#[macro_use]
extern crate rocket;
use dotenvy::{self, dotenv};
use mysql::Pool;
use rocket::{
    http::Status,
    serde::json::{serde_json::json, Value},
    Request,
};

mod application;
mod domain;
mod infrastructure;

#[catch(default)]
fn err_handler(status: Status, _req: &Request) -> Value {
    json!({
        "error": status.code,
        "message": status.reason()
    })
}
#[launch]
fn rocket() -> _ {
    dotenv().expect("Missing .env file.");
    let pool: Pool = Pool::new(std::env::var("MYSQL_URL").unwrap().as_str()).unwrap();
    let mut rocket = rocket::build().register("/", catchers![err_handler]);
    rocket = infrastructure::repository::manage(rocket, pool);
    rocket = application::actions::mount(rocket);
    rocket
}
