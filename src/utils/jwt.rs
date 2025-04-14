use crate::{app_state::AppState, config::Config, errors::AppError, errors::AppResult};
use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, HeaderMap, StatusCode},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{f64::MAX_EXP, sync::Arc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // subject (user ID)
    pub email: String,
    pub exp: usize, // expiration time (as UTC timestamp)
    pub iat: usize, // issued at (as UTC timestamp)
}

// use Lazy to initialize keys only once
static JWT_ENCODING_KEY: Lazy<EncodingKey> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    EncodingKey::from_secret(secret.as_ref())
});

static JWT_DECODING_KEY: Lazy<DecodingKey> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    DecodingKey::from_secret(secret.as_ref())
});

pub fn create_token(user_id: Uuid, email: &str) -> AppResult<String> {
    let now = Utc::now();
    let expires_in = Duration::days(7); // token valid for 7 days
    let exp = (now + expires_in).timestamp() as usize;
    let iat = now.timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        email: email.to_string(),
        exp,
        iat,
    };

    // encode(header, claims, key)
    encode(&Header::default(), &claims, &JWT_ENCODING_KEY).map_err(AppError::JwtError)
}

pub async fn validate_token(token: &str, config: &Config) -> AppResult<Claims> {
    let validation = Validation::default();
    decode::<Claims>(token, &JWT_DECODING_KEY, &validation)
        .map(|data| data.claims)
        .map_err(|e| {
            tracing::warn!("JWT validation failed: {}", e);
            AppError::JwtError(e)
        })
}
