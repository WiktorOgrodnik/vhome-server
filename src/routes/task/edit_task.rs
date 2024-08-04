use axum::extract::Path;
use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use crate::records::user::UserExtension;
use crate::{queries::task as queries, records::task::EditTask};

pub async fn edit_task(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
    Path(task_id): Path<i32>,
    Json(task): Json<EditTask>,
) -> Result<(), StatusCode> {
    queries::patch_task(&db, task_id, task, user.group_id.unwrap()).await?;

    Ok(())
}
