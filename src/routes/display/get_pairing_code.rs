use axum::{extract::State, http::StatusCode};
use chrono::{Duration, Utc};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use uuid::Uuid;

use crate::database::pairing_codes;

pub async fn get_pairing_code(State(db): State<DatabaseConnection>) -> Result<String, StatusCode> {
    let pairing_code = Uuid::new_v4().to_string();

    let pairing_code_row = pairing_codes::ActiveModel {
        pairing_code: Set(pairing_code.clone()),
        expiration_date: Set((Utc::now() + Duration::hours(1)).into()),
        ..Default::default()
    };

    pairing_code_row
        .insert(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(pairing_code)
}
