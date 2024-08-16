use axum::extract::Path;
use axum::Extension;
use axum::{extract::State, http::StatusCode};
use sea_orm::{DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::queries::task as queries;
use crate::records::user::UserExtension;

pub async fn set_assign(
    Extension(user): Extension<UserExtension>,
    Path((task_id, user_id)): Path<(i32, i32)>,
    State(db): State<DatabaseConnection>,
) -> Result<(), StatusCode> {
    let user = user.force_group_selected()?;

    let txn = db
        .begin_with_config(Some(IsolationLevel::ReadCommitted), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let task = queries::get_task(&txn, task_id, Some(user.group_id)).await?;
    let _ = queries::update_task(&txn, task).await?;

    let assigned = queries::is_task_assigned(&txn, task_id, user_id, Some(user.group_id)).await?;

    if assigned {
        return Err(StatusCode::BAD_REQUEST);
    }

    let _ = queries::add_task_assign(&txn, task_id, user_id).await?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

pub async fn set_unassing(
    Extension(user): Extension<UserExtension>,
    Path((task_id, user_id)): Path<(i32, i32)>,
    State(db): State<DatabaseConnection>,
) -> Result<(), StatusCode> {
    let user = user.force_group_selected()?;
    let txn = db
        .begin_with_config(Some(IsolationLevel::ReadCommitted), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let task = queries::get_task(&txn, task_id, Some(user.group_id)).await?;
    let _ = queries::update_task(&txn, task).await?;

    let _ = queries::get_task_assign(&txn, task_id, user_id).await?;
    let _ = queries::delete_task_assign(&txn, task_id, user_id).await?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
