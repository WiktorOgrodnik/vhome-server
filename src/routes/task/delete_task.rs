use axum::extract::Path;
use axum::Extension;
use axum::{extract::State, http::StatusCode};
use sea_orm::DatabaseConnection;

use crate::queries::task as queries;
use crate::records::user::UserExtension;

pub async fn delete_task(
    Extension(user): Extension<UserExtension>,
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<(), StatusCode> {
    let _ = queries::delete_task(&db, task_id, user.group_id.unwrap()).await?;

    Ok(())
}
