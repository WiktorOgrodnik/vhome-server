use axum::extract::Path;
use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use chrono::{Duration, Utc};
use sea_orm::{DatabaseConnection, TransactionTrait};

use crate::queries::device as queries;
use crate::records::device::{MeasurementsTimeRange, ResponseMeasurement};
use crate::records::user::UserExtension;

fn calculate_from_time(time_range: MeasurementsTimeRange) -> chrono::TimeDelta {
    match time_range {
        MeasurementsTimeRange::hour => Duration::hours(1),
        MeasurementsTimeRange::day => Duration::days(1),
        MeasurementsTimeRange::week => Duration::days(7),
        MeasurementsTimeRange::month => Duration::days(30),
    }
}

pub async fn get_measurements(
    Extension(user): Extension<UserExtension>,
    Path((device_id, time_range)): Path<(i32, MeasurementsTimeRange)>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseMeasurement>>, StatusCode> {
    let user = user.force_group_selected()?;
    let txn = db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = queries::get_device(&txn, device_id, Some(user.group_id)).await?;

    let measurements = queries::get_measurements(
        &txn,
        device_id,
        (Utc::now() - calculate_from_time(time_range)).into(),
        Utc::now().into(),
    )
    .await?
    .into_iter()
    .map(|elt| elt.into())
    .collect();

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(measurements))
}
