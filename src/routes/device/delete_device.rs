use axum::extract::Path;
use axum::Extension;
use axum::{extract::State, http::StatusCode};
use sea_orm::{DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::queries::device as queries;
use crate::records::user::UserExtension;

pub async fn delete_device(
    Extension(user): Extension<UserExtension>,
    Path(device_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<(), StatusCode> {
    let user = user.force_group_selected()?;

    let txn = db
        .begin_with_config(Some(IsolationLevel::ReadCommitted), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let device = queries::get_device(&txn, device_id, Some(user.group_id)).await?;
    let _ = queries::delete_device_measurements(&txn, device_id, None).await?;
    let _ = queries::delete_device(&txn, device).await?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
