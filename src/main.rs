#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message<'t> {
    description: &'t str,
    ok: bool
}
#[get("/")]
fn index<'a>() -> Json<Message<'a>> {
    let message = Message {
        description: "Hello world!",
        ok: true
    };
    Json(message)
}
#[get("/hola")]
fn hola<'a>() -> Json<Message<'a>> {
    let message = Message {
        description: "Hola mundo!",
        ok: true
    };
    Json(message)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, hola])
}