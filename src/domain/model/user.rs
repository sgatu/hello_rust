extern crate bcrypt;
extern crate chrono;

use bcrypt::{hash, verify, BcryptResult};
use chrono::{NaiveDateTime, Utc};
use email_address::*;
use passwords::analyzer;
use passwords::scorer;
use rand::Rng;
const PASSWORD_HASH_COST: u32 = 10;
#[derive(Debug, Eq, PartialEq)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created: NaiveDateTime,
}
#[derive(Debug, Eq, PartialEq)]
pub enum UserError {
    InvalidPassword,
    InvalidEmail,
}
const MINIMUM_PASSWORD_SCORE: f64 = 60f64;
impl User {
    pub fn new(name: &str, email: &str, password: &str) -> Result<Self, UserError> {
        let dt = Utc::now().naive_utc();
        match Self::validate_user(email, password) {
            Some(e) => Err(e),
            None => Ok(Self::from(
                rand::thread_rng().gen(),
                name,
                email,
                &Self::hash_password(password),
                dt,
            )),
        }
    }
    pub fn from(id: u64, name: &str, email: &str, password: &str, created: NaiveDateTime) -> Self {
        Self {
            id: id,
            name: name.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            created: created,
        }
    }
    pub fn verify_password(&self, password: &str) -> BcryptResult<bool> {
        verify(password, self.password.as_str())
    }
    fn hash_password(password: &str) -> String {
        return hash(password, PASSWORD_HASH_COST).unwrap();
    }
    fn validate_user(email: &str, password: &str) -> Option<UserError> {
        if !EmailAddress::is_valid(email) {
            return Some(UserError::InvalidEmail);
        }
        let pass_score = scorer::score(&analyzer::analyze(password));
        if pass_score < MINIMUM_PASSWORD_SCORE {
            return Some(UserError::InvalidPassword);
        }
        None
    }
}
