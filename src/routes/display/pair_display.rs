use axum::extract::State;
use axum::http::StatusCode;
use axum::Extension;
use sea_orm::{DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::queries::display::set_pairing_code_token;
use crate::queries::{display as queries, token::save_token};
use crate::records::{token::TokenType, user::UserExtension};
use crate::state::SecretWrapper;
use crate::utilities::token::create_token;

pub async fn pair_display(
    Extension(user): Extension<UserExtension>,
    State(secret): State<SecretWrapper>,
    State(db): State<DatabaseConnection>,
    body: String,
) -> Result<(), StatusCode> {
    let user = user.force_group_selected()?;

    let txn = db
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let pairing_code = queries::get_pairing_code(&txn, body, false).await?;

    let token = create_token(
        &secret.0,
        Some(user.id),
        TokenType::Normal,
        Some(user.group_id),
    )?;

    let token = save_token(&txn, Some(user.id), &token, TokenType::Normal).await?;
    let _ = set_pairing_code_token(&txn, pairing_code, token.id).await?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
