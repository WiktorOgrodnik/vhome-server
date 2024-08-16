use std::str::FromStr;

use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseTransaction, EntityTrait, QueryFilter, Set,
    TryIntoModel, Unchanged,
};

use crate::{
    database::device::{self, Entity as Device, Model as DeviceModel},
    queries::thermometer::add_thermometer,
    records::{
        device::{DeviceType, InsertDevice},
        token::TokenType,
    },
    utilities::token::create_token,
};

pub async fn get_device(
    txn: &DatabaseTransaction,
    device_id: i32,
) -> Result<DeviceModel, StatusCode> {
    Device::find_by_id(device_id)
        .one(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn get_devices(
    db: &DatabaseTransaction,
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

pub async fn add_device(
    txn: &DatabaseTransaction,
    device: InsertDevice,
    secret: String,
    user_id: i32,
    group_id: i32,
) -> Result<DeviceModel, StatusCode> {
    let device = device::ActiveModel {
        name: Set(device.name),
        dev_t: Set(DeviceType::from_str(&device.dev_t)?.into()),
        vgroup_id: Set(group_id),
        token: Set(create_token(
            &secret,
            Some(user_id),
            TokenType::Device,
            None,
        )?),
        initialized: Set(false),
        ..Default::default()
    };

    let device = save_active_device(txn, device).await?;
    let _ = create_related_structure(txn, &device).await?;
    Ok(device)
}

pub async fn update_device(
    txn: &DatabaseTransaction,
    device_id: i32,
) -> Result<DeviceModel, StatusCode> {
    let device_active = device::ActiveModel {
        id: Unchanged(device_id),
        initialized: Set(true),
        ..Default::default()
    };

    save_active_device(&txn, device_active).await
}

pub async fn save_active_device(
    txn: &DatabaseTransaction,
    device: device::ActiveModel,
) -> Result<DeviceModel, StatusCode> {
    device
        .save(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn create_related_structure(
    txn: &DatabaseTransaction,
    device: &DeviceModel,
) -> Result<(), StatusCode> {
    match device.dev_t.clone().into() {
        DeviceType::Thermometer => add_thermometer(txn, device).await.map(|_| ()),
        DeviceType::Other => Err(StatusCode::BAD_REQUEST),
    }
}
