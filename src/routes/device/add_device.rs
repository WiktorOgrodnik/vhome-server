use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::{IsolationLevel,DatabaseConnection, TransactionTrait};

use crate::queries::token::save_token;
use crate::records::token::TokenType;
use crate::records::user::UserExtension;
use crate::state::SecretWrapper;
use crate::{
    queries::device as queries,
    records::device::{InsertDevice, ResponseDeviceToken},
};

pub async fn add_device(
    Extension(user): Extension<UserExtension>,
    State(secret): State<SecretWrapper>,
    State(db): State<DatabaseConnection>,
    Json(device): Json<InsertDevice>,
) -> Result<(StatusCode, Json<ResponseDeviceToken>), StatusCode> {
    let user = user.force_group_selected()?;
    let txn = db
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let device = queries::add_device(&txn, device, secret.0, user.id, user.group_id).await?;
    let _ = save_token(&txn, None, &device.token, TokenType::Device).await?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(device.into())))
}
