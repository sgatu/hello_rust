#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::Json};

mod domain;
use domain::model::User;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message<'t> {
    description: &'t str,
    ok: bool
}
#[get("/")]
async fn index<'a>() -> Json<String> {
    let message = Message {
        description: "Hello world!",
        ok: true
    };
    Json(message.description.to_string())
}
#[get("/hola")]
fn hola<'a>() -> Json<Message<'a>> {
    let message = Message {
        description: "Hola mundo!",
        ok: true
    };
    Json(message)
}
#[get("/user")]
fn get_user() -> Json<User> {
    let user = User::new(
        "Stefan", 
        "stefan.gatu@gmail.com", 
        "abc"
    );
    Json(user)
}
#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, hola, get_user])
}