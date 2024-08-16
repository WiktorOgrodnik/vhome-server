use axum::extract::Path;
use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::{IsolationLevel,DatabaseConnection, TransactionTrait};

use crate::queries::thermometer as queries;
use crate::records::thermometer::ResponseThermometer;
use crate::records::user::UserExtension;

pub async fn get_thermometer(
    Extension(user): Extension<UserExtension>,
    Path(device_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseThermometer>, StatusCode> {
    let user = user.force_group_selected()?;
    let txn = db
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let thermometer = queries::get_thermometer(&txn, device_id, user.group_id)
        .await?
        .into();

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(thermometer))
}
