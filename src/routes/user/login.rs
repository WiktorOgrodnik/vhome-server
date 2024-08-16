use axum::{extract::State, http::StatusCode, Json};
use bcrypt::verify;
use sea_orm::{IsolationLevel,DatabaseConnection, TransactionTrait};

use crate::{
    queries::{token::save_token, user as queries},
    records::{
        token::TokenType,
        user::{RequestUser, ResponseUserLogin},
    },
    state::SecretWrapper,
    utilities::token::create_token,
};

pub async fn login(
    State(db): State<DatabaseConnection>,
    State(secret): State<SecretWrapper>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUserLogin>, StatusCode> {
    let txn = db
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = queries::get_user_by_username(&txn, &request_user.username).await?;

    if !verify(request_user.password, &user.passwd)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = create_token(&secret.0, Some(user.id), TokenType::Normal, None)?;
    let token = save_token(&txn, Some(user.id), &token, TokenType::Normal).await?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = ResponseUserLogin {
        id: user.id,
        username: user.login,
        token: token.token,
        group: None,
    };

    Ok(Json(response))
}
