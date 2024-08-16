use crate::database::thermometer::Model as ThermometerModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseThermometer {
    pub last_temp: Option<f32>,
    pub last_humidity: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateThermometer {
    pub token: String,
    pub current_temp: Option<f32>,
    pub current_humidity: Option<f32>,
}

impl From<ThermometerModel> for ResponseThermometer {
    fn from(value: ThermometerModel) -> Self {
        ResponseThermometer {
            last_temp: value.last_temp,
            last_humidity: value.last_humidity,
        }
    }
}
