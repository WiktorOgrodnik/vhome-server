use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::DatabaseConnection;

use crate::{
    database::vuser::Model as UserModel,
    queries::{token::get_normal_token_db, user as queries},
    records::{
        token::{Claims, TokenType},
        user::{GroupSelectedPayload, GroupUnselectedPayload, UserExtension},
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
    let header_token = headers
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let token: Claims =
        validate_token(&secret.0, header_token)?.force_token_t(TokenType::Normal)?;

    let _ = get_normal_token_db(&db, token.user_id.unwrap(), header_token)
        .await
        .map_err(|error| match error {
            StatusCode::BAD_REQUEST => StatusCode::UNAUTHORIZED,
            other => other,
        })?;

    let user: UserModel = queries::get_user_db(&db, token.user_id.unwrap()).await?;

    let extension = match token.related_id {
        None => UserExtension::GroupUnselected(GroupUnselectedPayload {
            id: user.id,
            username: user.login,
            token: header_token.to_string(),
        }),
        Some(group_id) => UserExtension::GroupSelected(GroupSelectedPayload {
            id: user.id,
            group_id,
            username: user.login,
            token: header_token.to_string(),
        }),
    };

    request.extensions_mut().insert(extension);
    Ok(next.run(request).await)
}
