use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    queries::{display as queries, group as group_queries, token as token_queries},
    records::user::ResponseUserLogin,
    state::SecretWrapper,
    utilities::token::validate_token,
};

pub async fn use_pairing_code(
    State(secret): State<SecretWrapper>,
    State(db): State<DatabaseConnection>,
    body: String,
) -> Result<Json<ResponseUserLogin>, StatusCode> {
    let txn = db
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let pairing_code = queries::get_pairing_code(&txn, body, true).await?;
    let token_id = pairing_code
        .token_id
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let token = token_queries::get_token_by_id(&txn, token_id).await?;

    let claims = validate_token(&secret.0, &token.token).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let _ = queries::delete_pairing_code(&txn, pairing_code).await?;

    let group = group_queries::get_group_by_id(
        &txn,
        claims.related_id.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?,
    )
    .await?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = ResponseUserLogin {
        id: claims.user_id.unwrap(),
        username: "display".to_owned(),
        token: token.token,
        group: Some(group.name),
    };

    Ok(Json(response))
}
