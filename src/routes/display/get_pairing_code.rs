use axum::{extract::State, http::StatusCode};
use chrono::{Duration, Utc};
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

use crate::queries::display as queries;

pub async fn get_pairing_code(State(db): State<DatabaseConnection>) -> Result<String, StatusCode> {
    let pairing_code = Uuid::new_v4().to_string();

    let txn = db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let pairing_code =
        queries::add_pairing_code(&txn, pairing_code, (Utc::now() + Duration::hours(1)).into())
            .await?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(pairing_code.pairing_code)
}
