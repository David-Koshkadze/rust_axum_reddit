use crate::errors::{AppError, AppResult};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use tracing::error;

pub async fn hash_passwrod(password: String) -> AppResult<String> {
    let password_bytes = password.into_bytes();

    // use spawn_blocking for CPU
    tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(&password_bytes, &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| {
                error!("Failed to hash password: {}", e);
                AppError::PasswordHashError(e)
            })
    })
    .await
    .map_err(|e| AppError::InternalServerError(format!("Password hashing task failed: {}", e)))?
    // handling join error
}

pub async fn verify_password(hash: &str, password: &str) -> AppResult<bool> {
    let hash_str = hash.to_string(); // Clone hash to move into the blocking task
    let password_bytes = password.to_string().into_bytes(); // Clone password

    tokio::task::spawn_blocking(move || {
        let parsed_hash = argon2::PasswordHash::new(&hash_str).map_err(|e| {
            error!("Failed to parse password hash: {}", e);
            AppError::PasswordHashError(e) // Convert to AppError
        })?;

        Argon2::default()
            .verify_password(&password_bytes, &parsed_hash)
            .map(|_| true) // Success means verification passed
            .or_else(|e| match e {
                argon2::password_hash::Error::Password => Ok(false), // specific error for mismatch
                _ => {
                    error!("Password verification error: {}", e);
                    Err(AppError::PasswordHashError(e)) // other errors
                }
            })
    })
    .await
    .map_err(|e| {
        AppError::InternalServerError(format!("Password verification task failed: {}", e))
    })?
}
