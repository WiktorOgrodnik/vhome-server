use crate::database::device::Model as DeviceModel;
use sea_orm::ActiveEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseDevice {
    pub id: i32,
    pub name: String,
    pub dev_t: String,
}

#[derive(Serialize, Deserialize)]
pub struct InsertDevice {
    pub name: String,
}

impl From<DeviceModel> for ResponseDevice {
    fn from(value: DeviceModel) -> Self {
        ResponseDevice {
            id: value.id,
            name: value.name,
            dev_t: value.dev_t.to_value(),
        }
    }
}
