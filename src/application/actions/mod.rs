use rocket::{Build, Rocket};

mod user;

use self::user::{get_me, greet, login, register, revoke_token};

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/user", routes![get_me, greet, register])
        .mount("/auth", routes![login, revoke_token])
}
