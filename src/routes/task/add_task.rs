use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::{DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::queries::taskset::has_permission;
use crate::records::user::UserExtension;
use crate::{
    queries::task as queries,
    records::task::{InsertTask, ResponseTask},
};

pub async fn add_task(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
    Json(task): Json<InsertTask>,
) -> Result<(StatusCode, Json<ResponseTask>), StatusCode> {
    let user = user.force_group_selected()?;

    let txn = db
        .begin_with_config(Some(IsolationLevel::ReadCommitted), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    has_permission(&txn, task.taskset_id, user.group_id).await?;
    let task = queries::create_task(&txn, task).await?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(task.into())))
}
