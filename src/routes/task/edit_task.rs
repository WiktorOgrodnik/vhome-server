use axum::extract::Path;
use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::{DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::records::user::UserExtension;
use crate::{queries::task as queries, records::task::EditTask};

pub async fn edit_task(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
    Path(task_id): Path<i32>,
    Json(edited_task): Json<EditTask>,
) -> Result<(), StatusCode> {
    let user = user.force_group_selected()?;

    let txn = db
        .begin_with_config(Some(IsolationLevel::ReadCommitted), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let task = queries::get_task(&txn, task_id, Some(user.group_id)).await?;
    queries::patch_task(&txn, task, edited_task).await?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
