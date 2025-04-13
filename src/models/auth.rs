use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Username must be between 3 and 50 characters"
    ))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(length(min = 1, message = "Login identifier cannot be empty"))]
    pub login: String, // can be username or email
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}
