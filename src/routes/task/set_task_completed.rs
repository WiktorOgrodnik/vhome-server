use axum::extract::Path;
use axum::Extension;
use axum::{extract::State, http::StatusCode};
use sea_orm::DatabaseConnection;

use crate::queries::task as queries;
use crate::records::user::UserExtension;

pub async fn set_completed(
    Extension(user): Extension<UserExtension>,
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<(), StatusCode> {
    queries::change_completed(&db, task_id, user.group_id.unwrap(), true).await?;
    Ok(())
}

pub async fn set_uncompleted(
    Extension(user): Extension<UserExtension>,
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<(), StatusCode> {
    queries::change_completed(&db, task_id, user.group_id.unwrap(), false).await?;
    Ok(())
}
