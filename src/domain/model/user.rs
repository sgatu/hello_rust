extern crate bcrypt;
extern crate chrono;

use super::formatter::formatter;

use bcrypt::hash;
use rocket::serde::{Serialize, Deserialize}; 
use rand::Rng;
use chrono::{NaiveDateTime, Utc};

const PASSWORD_HASH_COST: u32 = 10;



#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[readonly::make]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(with = "formatter::datetime")]
    pub created: NaiveDateTime,
}

impl User {
    pub fn new(name: &str, email: &str, password: &str) -> Self {
        let dt = Utc::now().naive_utc();
        Self {
            id: rand::thread_rng().gen(),
            name: name.to_string(),
            email: email.to_string(),
            password: Self::hash_password(password),
            created: dt
        }
    }
    pub fn from(id: u64, name: &str, email: &str, password: &str, created: NaiveDateTime) -> Self {
        Self {
            id: id,
            name: name.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            created: created
        }
    }

    fn hash_password(password: &str) -> String {
        return hash(password, PASSWORD_HASH_COST).unwrap();
    }
}
