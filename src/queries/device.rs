use std::str::FromStr;

use axum::http::StatusCode;
use chrono::Utc;
use sea_orm::{
    prelude::DateTimeWithTimeZone, ActiveModelTrait, ColumnTrait, Condition, DatabaseTransaction,
    EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Set, TryIntoModel, Unchanged,
};

use crate::{
    database::{
        device::{self, Entity as Device, Model as DeviceModel},
        device_measurements::{self, Entity as DeviceMeasurement, Model as DeviceMeasurementModel},
        sea_orm_active_enums::DeviceType as DatabaseDeviceType,
    },
    queries::thermometer::{add_thermometer, delete_thermometer},
    records::{
        device::{DeviceType, EditDevice, InsertDevice},
        token::TokenType,
    },
    utilities::token::create_token,
};

pub async fn get_device(
    txn: &DatabaseTransaction,
    device_id: i32,
    group_id: Option<i32>,
) -> Result<DeviceModel, StatusCode> {
    let condition = if let Some(id) = group_id {
        Condition::all().add(device::Column::VgroupId.eq(id))
    } else {
        Condition::all()
    };

    Device::find_by_id(device_id)
        .filter(condition)
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

pub async fn get_measurements(
    txn: &DatabaseTransaction,
    device_id: i32,
    time_from: DateTimeWithTimeZone,
    time_to: DateTimeWithTimeZone,
) -> Result<Vec<DeviceMeasurementModel>, StatusCode> {
    DeviceMeasurement::find()
        .filter(
            Condition::all()
                .add(device_measurements::Column::DeviceId.eq(device_id))
                .add(device_measurements::Column::MeasurementTime.gte(time_from))
                .add(device_measurements::Column::MeasurementTime.lte(time_to)),
        )
        .all(txn)
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
    create_related_structure(txn, &device).await?;
    Ok(device)
}

pub async fn patch_device(
    txn: &DatabaseTransaction,
    device: DeviceModel,
    edited_device: EditDevice,
) -> Result<DeviceModel, StatusCode> {
    let mut active_device = device.into_active_model();

    active_device.name = Set(edited_device.name);

    save_active_device(txn, active_device).await
}

pub async fn update_device(
    txn: &DatabaseTransaction,
    device_id: i32,
) -> Result<DeviceModel, StatusCode> {
    let device_active = device::ActiveModel {
        id: Unchanged(device_id),
        initialized: Set(true),
        last_updated: Set(Utc::now().into()),
        ..Default::default()
    };

    save_active_device(txn, device_active).await
}

pub async fn delete_device(
    txn: &DatabaseTransaction,
    device: DeviceModel,
) -> Result<sea_orm::DeleteResult, StatusCode> {
    match device.dev_t {
        DatabaseDeviceType::Thermometer => {
            delete_thermometer(txn, device.id).await?;
        }
        _ => (),
    };

    device
        .delete(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
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

pub async fn add_device_measurement(
    txn: &DatabaseTransaction,
    device_id: i32,
    label: &str,
    value: f32,
) -> Result<DeviceMeasurementModel, StatusCode> {
    let measurement = device_measurements::ActiveModel {
        device_id: Set(device_id),
        measurement_label: Set(label.to_string()),
        measurement_value: Set(value),
        measurement_time: Set(Utc::now().into()),
    };

    measurement
        .insert(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_device_measurements(
    txn: &DatabaseTransaction,
    device_id: i32,
    older_then: Option<DateTimeWithTimeZone>,
) -> Result<sea_orm::DeleteResult, StatusCode> {
    let condition = if let Some(older_then) = older_then {
        Condition::all()
            .add(device_measurements::Column::DeviceId.eq(device_id))
            .add(device_measurements::Column::MeasurementTime.lte(older_then))
    } else {
        Condition::all().add(device_measurements::Column::DeviceId.eq(device_id))
    };

    DeviceMeasurement::delete_many()
        .filter(condition)
        .exec(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
