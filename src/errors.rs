use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] std::env::VarError),

    #[error("Migration error: {0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),

    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Password hashing error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),

    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("Missing credentials")]
    MissingCredentials,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),

    #[error("Internal server error")]
    InternalServerError(String), // unexpected errors
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::SqlxError(ref e) => {
                tracing::error!("SQLx error {:?}", e);
                // check for specific DB errors
                if let Some(db_err) = e.as_database_error() {
                    if db_err.is_unique_violation() {
                        (
                            StatusCode::CONFLICT,
                            format!("Resource already exists. {}", db_err.message()),
                        )
                    } else if db_err.is_foreign_key_violation() {
                        (
                            StatusCode::BAD_REQUEST,
                            format!(
                                "Invalid reference to another resource. {}",
                                db_err.message()
                            ),
                        )
                    } else {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Database error".to_string(),
                        )
                    }
                } else {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Database error".to_string(),
                    )
                }
            }
            AppError::ConfigError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Configuration error".to_string(),
            ),
            AppError::MigrateError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database migration error".to_string(),
            ),
            AppError::AuthError(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::PasswordHashError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error (hash)".to_string(),
            ),
            AppError::JwtError(_) => (
                StatusCode::UNAUTHORIZED,
                "Invalid or expired token".to_string(),
            ),
            AppError::MissingCredentials => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::ValidationError(_) => {
                // TODO: implement detailed errors
                (StatusCode::BAD_REQUEST, "Validationo failed".to_string())
            }
            AppError::InternalServerError(msg) => {
                tracing::error!("Internal server error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };
        let body = Json(json!({"error": error_message}));
        (status, body).into_response()
    }
}

// helper type for handlers
pub type AppResult<T> = Result<T, AppError>;
