use axum::http::StatusCode;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use crate::database::device::{self, Entity as Device, Model as DeviceModel};

pub async fn get_all_devices(
    db: &DatabaseConnection,
    group_id: Option<i32>,
) -> Result<Vec<DeviceModel>, StatusCode> {
    let condition = if let Some(id) = group_id {
        Condition::all().add(device::Column::VgroupId.eq(id))
    } else {
        Condition::all()
    };

    Device::find()
        .filter(condition)
        .all(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
