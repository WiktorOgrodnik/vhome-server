use crate::database::thermometer::Model as ThermometerModel;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseThermometer {
    pub last_temp: Option<f32>,
    pub last_updated: Option<DateTimeWithTimeZone>,
}

impl From<ThermometerModel> for ResponseThermometer {
    fn from(value: ThermometerModel) -> Self {
        ResponseThermometer {
            last_temp: value.last_temp,
            last_updated: value.last_updated,
        }
    }
}
