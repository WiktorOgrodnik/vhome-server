use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "device_type", rename_all = "lowercase")]
pub enum DeviceType {
    Thermometer,
    Other,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Data {
    pub id: i32,
    pub group_id: i32,
    pub name: String,
    pub dev_t: DeviceType,
}

impl Data {
    pub async fn all(db: &PgPool, group_id: i32) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, group_id, name, dev_t AS "dev_t: _" FROM device WHERE device.group_id = $1
            "#,
            group_id,
        )
        .fetch_all(db)
        .await
    }

    pub async fn get_guarded(db: &PgPool, dev_id: i32, group_id: i32) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(Self,
            r#"
            SELECT id, group_id, name, dev_t AS "dev_t: _" FROM device WHERE device.id = $1 AND device.group_id = $2
            "#,
            dev_id,
            group_id,
        ).fetch_one(db).await
    }
}
