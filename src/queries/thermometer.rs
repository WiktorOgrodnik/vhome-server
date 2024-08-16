use axum::http::StatusCode;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter, Set,
    TryIntoModel, Unchanged,
};

use crate::{
    database::{
        device::{self, Entity as Device, Model as DeviceModel},
        thermometer::{self, Entity as Thermometer, Model as ThermometerModel},
    },
    records::thermometer::UpdateThermometer,
};

pub async fn get_thermometer(
    txn: &DatabaseTransaction,
    device_id: i32,
    group_id: i32,
) -> Result<ThermometerModel, StatusCode> {
    let thermometer = Some(
        Device::find()
            .filter(device::Column::Id.eq(device_id))
            .find_with_related(Thermometer)
            .all(txn)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .into_iter()
            .next()
            .ok_or(StatusCode::NOT_FOUND)?,
    )
    .filter(|(device, _)| device.vgroup_id == group_id)
    .ok_or(StatusCode::FORBIDDEN)?
    .1
    .into_iter()
    .next()
    .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(thermometer)
}

pub async fn get_thermometer_by_token(
    txn: &DatabaseTransaction,
    token: &str,
) -> Result<ThermometerModel, StatusCode> {
    Ok(Device::find()
        .filter(device::Column::Token.eq(token))
        .find_with_related(Thermometer)
        .all(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .first()
        .ok_or(StatusCode::NOT_FOUND)?
        .clone()
        .1
        .first()
        .ok_or(StatusCode::NOT_FOUND)?
        .clone())
}

pub async fn add_thermometer(
    txn: &DatabaseTransaction,
    device: &DeviceModel,
) -> Result<ThermometerModel, StatusCode> {
    let thermometer = thermometer::ActiveModel {
        device_id: Set(device.id),
        ..Default::default()
    };

    thermometer
        .insert(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn patch_thermometer(
    txn: &DatabaseTransaction,
    device_id: i32,
    data: UpdateThermometer,
) -> Result<ThermometerModel, StatusCode> {
    let thermometer_active = thermometer::ActiveModel {
        device_id: Unchanged(device_id),
        last_temp: Set(data.current_temp),
        last_humidity: Set(data.current_humidity),
        last_updated: Set(Utc::now().into()),
    };

    save_active_thermometer(txn, thermometer_active).await
}

pub async fn save_active_thermometer(
    txn: &DatabaseTransaction,
    thermometer: thermometer::ActiveModel,
) -> Result<ThermometerModel, StatusCode> {
    thermometer
        .save(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
