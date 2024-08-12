use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::DatabaseConnection;

use crate::{
    queries::{token::get_normal_token, user as queries},
    records::{
        token::{Claims, TokenType},
        user::UserExtension,
    },
    state::SecretWrapper,
    utilities::token::validate_token,
};

pub async fn requires_authentication(
    State(db): State<DatabaseConnection>,
    State(secret): State<SecretWrapper>,
    headers: HeaderMap,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let header_token = if let Some(token) = headers.get("Authorization") {
        token.to_str().map_err(|_| StatusCode::BAD_REQUEST)?
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let token: Claims =
        validate_token(&secret.0, header_token)?.force_token_t(TokenType::Normal)?;
    let _ = get_normal_token(&db, token.user_id.unwrap(), header_token)
        .await
        .map_err(|error| match error {
            StatusCode::BAD_REQUEST => StatusCode::UNAUTHORIZED,
            other => other,
        })?;
    let mut user: UserExtension = queries::find_by_id(&db, token.user_id.unwrap())
        .await?
        .into();

    user.group_id = token.related_id;
    header_token.clone_into(&mut user.token);

    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}
