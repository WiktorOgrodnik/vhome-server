use axum::{extract::State, http::StatusCode, Json};
use bcrypt::verify;
use sea_orm::DatabaseConnection;

use crate::{
    queries::token::save_token,
    queries::user as queries,
    records::user::{RequestUser, ResponseUser},
    state::SecretWrapper,
    utilities::token::create_token,
};

pub async fn login(
    State(db): State<DatabaseConnection>,
    State(secret): State<SecretWrapper>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let user = queries::find_by_username(&db, request_user.username).await?;

    if !verify(request_user.password, &user.passwd)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = create_token(&secret.0, user.id, None)?;
    let token = save_token(&db, user.id, &token).await?;

    let response = ResponseUser {
        id: user.id,
        username: user.login,
        token: token.token,
    };

    Ok(Json(response))
}
