use axum::extract::Path;
use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::{DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::records::user::UserExtension;
use crate::{queries::taskset as queries, records::taskset::InsertTaskset};

pub async fn edit_taskset(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
    Path(taskset_id): Path<i32>,
    Json(edited_taskset): Json<InsertTaskset>,
) -> Result<(), StatusCode> {
    let user = user.force_group_selected()?;

    let txn = db
        .begin_with_config(Some(IsolationLevel::ReadCommitted), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let taskset = queries::get_taskset(&txn, taskset_id, user.group_id).await?;
    let _ = queries::patch_taskset(&txn, taskset, edited_taskset).await?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
