use axum::extract::Path;
use axum::Extension;
use axum::{extract::State, http::StatusCode};
use sea_orm::{DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::queries::task as queries;
use crate::records::user::UserExtension;

pub async fn delete_task(
    Extension(user): Extension<UserExtension>,
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<(), StatusCode> {
    let user = user.force_group_selected()?;

    let txn = db
        .begin_with_config(Some(IsolationLevel::ReadCommitted), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = queries::get_task(&txn, task_id, Some(user.group_id)).await?;
    let _ = queries::delete_task_assigns(&txn, task_id).await?;
    let _ = queries::delete_task_by_id(&txn, task_id).await?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
