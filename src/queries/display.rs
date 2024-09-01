use axum::http::StatusCode;
use chrono::Utc;
use sea_orm::{
    prelude::DateTimeWithTimeZone, ActiveModelTrait, ColumnTrait, Condition, DatabaseTransaction,
    EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Set, TryIntoModel,
};

use crate::database::pairing_codes::{self, Entity as PairingCode, Model as PairingCodeModel};

pub async fn get_pairing_code(
    txn: &DatabaseTransaction,
    code: String,
    is_token: bool,
) -> Result<PairingCodeModel, StatusCode> {
    let condition = if is_token {
        Condition::all()
            .add(pairing_codes::Column::ExpirationDate.gte(Utc::now()))
            .add(pairing_codes::Column::TokenId.is_not_null())
    } else {
        Condition::all().add(pairing_codes::Column::ExpirationDate.gte(Utc::now()))
    };

    PairingCode::find_by_id(code)
        .filter(condition)
        .one(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .filter(|row| row.expiration_date >= Utc::now())
        .filter(|row| !is_token || row.token_id.is_some())
        .ok_or(StatusCode::BAD_REQUEST)
}

pub async fn add_pairing_code(
    txn: &DatabaseTransaction,
    pairing_code: String,
    expiration_date: DateTimeWithTimeZone,
) -> Result<PairingCodeModel, StatusCode> {
    let pairing_code = pairing_codes::ActiveModel {
        pairing_code: Set(pairing_code),
        expiration_date: Set(expiration_date),
        ..Default::default()
    };

    insert_active_pairing_code(txn, pairing_code).await
}

pub async fn set_pairing_code_token(
    txn: &DatabaseTransaction,
    pairing_code: PairingCodeModel,
    token_id: i32,
) -> Result<PairingCodeModel, StatusCode> {
    let mut pairing_code = pairing_code.into_active_model();

    pairing_code.token_id = Set(Some(token_id));

    save_active_pairing_code(txn, pairing_code).await
}

pub async fn delete_pairing_code(
    txn: &DatabaseTransaction,
    pairing_code: PairingCodeModel,
) -> Result<sea_orm::DeleteResult, StatusCode> {
    pairing_code
        .delete(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn insert_active_pairing_code(
    txn: &DatabaseTransaction,
    pairing_code: pairing_codes::ActiveModel,
) -> Result<PairingCodeModel, StatusCode> {
    pairing_code
        .insert(txn)
        .await
        .map_err(|_| StatusCode::GONE)?
        .try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn save_active_pairing_code(
    txn: &DatabaseTransaction,
    pairing_code: pairing_codes::ActiveModel,
) -> Result<PairingCodeModel, StatusCode> {
    pairing_code
        .save(txn)
        .await
        .map_err(|_| StatusCode::GONE)?
        .try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
