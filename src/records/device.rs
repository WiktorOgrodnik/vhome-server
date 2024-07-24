use std::str::FromStr;

use crate::database::{
    device::Model as DeviceModel, sea_orm_active_enums::DeviceType as DatabaseDeviceType,
};
use axum::http::StatusCode;
use sea_orm::ActiveEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum DeviceType {
    Thermometer,
    Other,
}

impl Into<DatabaseDeviceType> for DeviceType {
    fn into(self) -> DatabaseDeviceType {
        match self {
            DeviceType::Thermometer => DatabaseDeviceType::Thermometer,
            DeviceType::Other => DatabaseDeviceType::Other,
        }
    }
}

impl Into<DeviceType> for DatabaseDeviceType {
    fn into(self) -> DeviceType {
        match self {
            DatabaseDeviceType::Thermometer => DeviceType::Thermometer,
            DatabaseDeviceType::Other => DeviceType::Other,
        }
    }
}

impl FromStr for DeviceType {
    type Err = StatusCode;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "thermometer" => Ok(DeviceType::Thermometer),
            _ => Err(StatusCode::BAD_REQUEST),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDevice {
    pub id: i32,
    pub name: String,
    pub dev_t: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDeviceToken {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct InsertDevice {
    pub name: String,
    pub dev_t: String,
}

impl From<DeviceModel> for ResponseDevice {
    fn from(value: DeviceModel) -> Self {
        ResponseDevice {
            id: value.id,
            name: value.name,
            dev_t: value.dev_t.to_value(),
            token: value.token,
        }
    }
}

impl From<DeviceModel> for ResponseDeviceToken {
    fn from(value: DeviceModel) -> Self {
        ResponseDeviceToken { token: value.token }
    }
}
