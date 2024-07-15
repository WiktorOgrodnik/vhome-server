use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::records::token::Claims;

pub fn create_token(
    secret: &str,
    user_id: i32,
    group_id: Option<i32>,
) -> Result<String, StatusCode> {
    let exp = (Utc::now() + Duration::days(7)).timestamp() as usize;

    let claims = Claims {
        exp,
        user_id,
        group_id,
    };

    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&token_header, &claims, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn validate_token(secret: &str, token: &str) -> Result<Claims, StatusCode> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);
    let token = decode::<Claims>(token, &key, &validation).map_err(|error| match error.kind() {
        jsonwebtoken::errors::ErrorKind::InvalidToken => StatusCode::BAD_REQUEST,
        jsonwebtoken::errors::ErrorKind::InvalidSignature
        | jsonwebtoken::errors::ErrorKind::ExpiredSignature => StatusCode::UNAUTHORIZED,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(token.claims)
}
