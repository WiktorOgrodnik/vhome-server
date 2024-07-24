use axum::http::StatusCode;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::database::{
    device::{self, Entity as Device},
    thermometer::{Entity as Thermometer, Model as ThermometerModel},
};

pub async fn get_thermometer(
    db: &DatabaseConnection,
    device_id: i32,
    group_id: i32,
) -> Result<ThermometerModel, StatusCode> {
    let thermometer = Some(
        Device::find()
            .filter(device::Column::Id.eq(device_id))
            .find_with_related(Thermometer)
            .all(db)
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
    db: &DatabaseConnection,
    token: &str,
) -> Result<ThermometerModel, StatusCode> {
    Ok(Device::find()
        .filter(device::Column::Token.eq(token))
        .find_with_related(Thermometer)
        .all(db)
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
