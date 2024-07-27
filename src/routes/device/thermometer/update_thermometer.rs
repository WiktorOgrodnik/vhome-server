use axum::Json;
use axum::{extract::State, http::StatusCode};
use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, Set, Unchanged};

use crate::database::{
    device::{self, Entity as Device},
    thermometer,
};
use crate::queries::device::{save_active_device, save_active_thermometer};
use crate::queries::thermometer::get_thermometer_by_token;
use crate::queries::token::get_device_token;
use crate::records::thermometer::UpdateThermometer;
use crate::records::token::{Claims, TokenType};
use crate::state::SecretWrapper;
use crate::utilities::token::validate_device_token;

pub async fn update_thermometer(
    State(db): State<DatabaseConnection>,
    State(secret): State<SecretWrapper>,
    Json(data): Json<UpdateThermometer>,
) -> Result<(), StatusCode> {
    let _: Claims =
        validate_device_token(&secret.0, &data.token)?.force_token_t(TokenType::Device)?;

    let token_model = get_device_token(&db, &data.token)
        .await
        .map_err(|error| match error {
            StatusCode::BAD_REQUEST => StatusCode::UNAUTHORIZED,
            other => other,
        })?;

    let thermometer = get_thermometer_by_token(&db, &token_model.token).await?;
    let device = Device::find_by_id(thermometer.device_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let thermometer_active = thermometer::ActiveModel {
        device_id: Unchanged(device.id),
        last_temp: Set(data.current_temp),
        last_humidity: Set(data.current_humidity),
        last_updated: Set(Utc::now().into()),
    };

    let device_active = device::ActiveModel {
        id: Unchanged(device.id),
        initialized: Set(true),
        ..Default::default()
    };

    save_active_device(&db, device_active).await?;
    save_active_thermometer(&db, thermometer_active).await?;

    Ok(())
}
