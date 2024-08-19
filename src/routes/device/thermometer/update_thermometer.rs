use axum::Json;
use axum::{extract::State, http::StatusCode};
use sea_orm::{DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    queries::{device as device_queries, thermometer as queries, token::get_device_token},
    records::{
        thermometer::UpdateThermometer,
        token::{Claims, TokenType},
    },
    state::SecretWrapper,
    utilities::token::validate_device_token,
};

pub async fn update_thermometer(
    State(db): State<DatabaseConnection>,
    State(secret): State<SecretWrapper>,
    Json(data): Json<UpdateThermometer>,
) -> Result<(), StatusCode> {
    let txn = db
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let _: Claims =
        validate_device_token(&secret.0, &data.token)?.force_token_t(TokenType::Device)?;

    let token_model = get_device_token(&txn, &data.token)
        .await
        .map_err(|error| match error {
            StatusCode::BAD_REQUEST => StatusCode::UNAUTHORIZED,
            other => other,
        })?;

    let thermometer = queries::get_thermometer_by_token(&txn, &token_model.token).await?;
    let device = device_queries::get_device(&txn, thermometer.device_id, None).await?;

    let _ = device_queries::update_device(&txn, device.id).await?;
    let _ = queries::patch_thermometer(&txn, device.id, data).await?;

    if let Some(last_temp) = thermometer.last_temp {
        device_queries::add_device_measurement(&txn, device.id, "last_temp", last_temp).await?;
    }

    if let Some(last_humidity) = thermometer.last_humidity {
        device_queries::add_device_measurement(&txn, device.id, "last_humidity", last_humidity)
            .await?;
    }

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
