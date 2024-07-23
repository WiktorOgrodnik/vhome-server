use axum::extract::Path;
use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use crate::queries::thermometer as queries;
use crate::records::thermometer::ResponseThermometer;
use crate::records::user::UserExtension;

pub async fn get_thermometer(
    Extension(user): Extension<UserExtension>,
    Path(device_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseThermometer>, StatusCode> {
    let thermometer = queries::get_thermometer(&db, device_id, user.group_id.unwrap())
        .await?
        .into();

    Ok(Json(thermometer))
}
