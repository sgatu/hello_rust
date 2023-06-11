#[cfg(test)]
use crate::domain::model::UserError;

const BAD_EMAIL: &str = "bad_email_address";
const BAD_PASSWORD: &str = "bad_pass";
const GOOD_EMAIL: &str = "good@email.com";
const GOOD_PASSWORD: &str = "str0ng_Password_h3re";
#[test]
fn check_user_creation() {
    use crate::domain::model::User;

    let bad_email = User::new("User name", BAD_EMAIL, GOOD_PASSWORD);
    assert!(
        bad_email.is_err() && matches!(bad_email.unwrap_err(), UserError::InvalidEmail),
        "User created with invalid email {} or invalid error type returned.",
        BAD_EMAIL
    );
    let bad_password = User::new("User name", GOOD_EMAIL, BAD_PASSWORD);
    assert!(
        bad_password.is_err() && matches!(bad_password.unwrap_err(), UserError::InvalidPassword),
        "User created with invalid password {} or invalid error type returned.",
        BAD_PASSWORD
    );

    let good_user = User::new("User name", GOOD_EMAIL, GOOD_PASSWORD);
    assert!(
        good_user.is_ok(),
        "Cannot create user with valid data. Email {}, Password {}",
        BAD_EMAIL,
        BAD_PASSWORD
    );
    let user = good_user.unwrap();
    assert!(
        user.verify_password(GOOD_PASSWORD).is_ok(),
        "Could not validate hashed password"
    )
}
