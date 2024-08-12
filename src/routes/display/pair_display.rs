use axum::extract::State;
use axum::http::StatusCode;
use axum::Extension;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set, TransactionTrait,
};

use crate::database::pairing_codes::Entity as PairingCodes;
use crate::queries::token::save_token_txn;
use crate::records::{token::TokenType, user::UserExtension};
use crate::state::SecretWrapper;
use crate::utilities::token::create_token;

pub async fn pair_display(
    Extension(user): Extension<UserExtension>,
    State(secret): State<SecretWrapper>,
    State(db): State<DatabaseConnection>,
    body: String,
) -> Result<(), StatusCode> {
    let txn = db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    dbg!(body.clone());

    let mut pairing_code_row = PairingCodes::find_by_id(body)
        .one(&txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .filter(|row| row.expiration_date >= Utc::now())
        .ok_or(StatusCode::BAD_REQUEST)?
        .into_active_model();

    let token = create_token(&secret.0, Some(user.id), TokenType::Normal, user.group_id)?;
    let token = save_token_txn(&txn, Some(user.id), &token, TokenType::Normal).await?;

    pairing_code_row.token_id = Set(Some(token.id));

    dbg!(token.id);

    pairing_code_row
        .save(&txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
