use axum::extract::Path;
use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::{DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::records::user::UserExtension;
use crate::{queries::device as queries, records::device::EditDevice};

pub async fn edit_device(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
    Path(device_id): Path<i32>,
    Json(edited_device): Json<EditDevice>,
) -> Result<(), StatusCode> {
    let user = user.force_group_selected()?;

    let txn = db
        .begin_with_config(Some(IsolationLevel::ReadCommitted), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let device = queries::get_device(&txn, device_id, Some(user.group_id)).await?;
    let _ = queries::patch_device(&txn, device, edited_device).await?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
