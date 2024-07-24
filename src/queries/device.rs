use std::str::FromStr;

use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TryIntoModel,
};

use crate::{
    database::{
        device::{self, Entity as Device, Model as DeviceModel},
        thermometer::{self, Model as ThermometerModel},
    },
    records::{
        device::{DeviceType, InsertDevice},
        token::TokenType,
    },
    utilities::token::create_token,
};

use super::token::save_token;

pub async fn get_all_devices(
    db: &DatabaseConnection,
    group_id: Option<i32>,
) -> Result<Vec<DeviceModel>, StatusCode> {
    let condition = if let Some(id) = group_id {
        Condition::all()
            .add(device::Column::VgroupId.eq(id))
            .add(device::Column::Initialized.eq(true))
    } else {
        Condition::all()
    };

    Device::find()
        .filter(condition)
        .all(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn create_device(
    db: &DatabaseConnection,
    device: InsertDevice,
    secret: String,
    user_id: i32,
    group_id: i32,
) -> Result<DeviceModel, StatusCode> {
    let device = device::ActiveModel {
        name: Set(device.name),
        dev_t: Set(DeviceType::from_str(&device.dev_t)?.into()),
        vgroup_id: Set(group_id),
        token: Set(create_token(&secret, user_id, TokenType::Device, None)?),
        initialized: Set(false),
        ..Default::default()
    };

    let device = save_active_device(db, device).await?;
    save_token(db, user_id, &device.token, TokenType::Device).await?;
    create_related_structure(db, &device).await?;
    Ok(device)
}

pub async fn save_active_device(
    db: &DatabaseConnection,
    device: device::ActiveModel,
) -> Result<DeviceModel, StatusCode> {
    device
        .save(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn create_related_structure(
    db: &DatabaseConnection,
    device: &DeviceModel,
) -> Result<(), StatusCode> {
    match device.dev_t.clone().into() {
        DeviceType::Thermometer => create_thermometer(db, device).await.map(|_| ()),
        DeviceType::Other => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn create_thermometer(
    db: &DatabaseConnection,
    device: &DeviceModel,
) -> Result<ThermometerModel, StatusCode> {
    let thermometer = thermometer::ActiveModel {
        device_id: Set(device.id),
        ..Default::default()
    };

    thermometer
        .insert(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn save_active_thermometer(
    db: &DatabaseConnection,
    thermometer: thermometer::ActiveModel,
) -> Result<ThermometerModel, StatusCode> {
    thermometer
        .save(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
