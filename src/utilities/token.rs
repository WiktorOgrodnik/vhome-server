use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::records::token::{Claims, TokenType};

pub fn create_token(
    secret: &str,
    user_id: Option<i32>,
    token_t: TokenType,
    related_id: Option<i32>,
) -> Result<String, StatusCode> {
    let exp = (Utc::now() + Duration::days(7)).timestamp() as usize;

    let claims = Claims {
        exp,
        user_id,
        token_t,
        related_id,
    };

    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&token_header, &claims, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn validate_token(secret: &str, token: &str) -> Result<Claims, StatusCode> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let mut validation = Validation::new(Algorithm::HS256);
    let token = decode::<Claims>(token, &key, &validation).map_err(|error| match error.kind() {
        jsonwebtoken::errors::ErrorKind::InvalidToken => StatusCode::BAD_REQUEST,
        jsonwebtoken::errors::ErrorKind::InvalidSignature
        | jsonwebtoken::errors::ErrorKind::ExpiredSignature => StatusCode::UNAUTHORIZED,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(token.claims)
}

pub fn validate_device_token(secret: &str, token: &str) -> Result<Claims, StatusCode> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false;
    let token = decode::<Claims>(token, &key, &validation).map_err(|error| match error.kind() {
        jsonwebtoken::errors::ErrorKind::InvalidToken => StatusCode::BAD_REQUEST,
        jsonwebtoken::errors::ErrorKind::InvalidSignature => StatusCode::UNAUTHORIZED,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(token.claims)
}
