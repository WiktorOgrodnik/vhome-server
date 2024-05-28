use sqlx::types::chrono;
use serde::{Deserialize, Serialize};

use sqlx::PgPool;

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Thermometer {
    pub device_id: i32,
    pub last_temp: Option<f32>,
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>, 
}

impl Thermometer {
    pub async fn get(db: &PgPool, device_id: i32) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(Self,
            "
            SELECT * FROM thermometer WHERE thermometer.device_id = $1
            ",
            device_id,
        ).fetch_one(db).await
    }
}
