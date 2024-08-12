use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait, TransactionTrait};

use crate::database::pairing_codes::Entity as PairingCodes;
use crate::database::tokens::Entity as Tokens;
use crate::database::vgroup::Entity as Group;
use crate::records::user::ResponseUserLogin;
use crate::state::SecretWrapper;
use crate::utilities::token::validate_token;

pub async fn use_pairing_code(
    State(secret): State<SecretWrapper>,
    State(db): State<DatabaseConnection>,
    body: String,
) -> Result<Json<ResponseUserLogin>, StatusCode> {
    let txn = db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let pairing_code_row = PairingCodes::find_by_id(body)
        .one(&txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .filter(|row| row.expiration_date > Utc::now())
        .filter(|row| row.token_id.is_some())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let token = Tokens::find_by_id(
        pairing_code_row
            .token_id
            .expect("token id is required here"),
    )
    .one(&txn)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::BAD_REQUEST)?;

    let claims = validate_token(&secret.0, &token.token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let _ = pairing_code_row
        .delete(&txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let group = Group::find_by_id(claims.related_id.unwrap())
        .one(&txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::BAD_REQUEST)?;

    let _ = txn
        .commit()
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
