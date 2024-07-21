use axum::extract::Path;
use axum::Extension;
use axum::{extract::State, http::StatusCode};
use sea_orm::DatabaseConnection;

use crate::queries::taskset as queries;
use crate::records::user::UserExtension;

pub async fn delete_taskset(
    Extension(user): Extension<UserExtension>,
    Path(taskset_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<(), StatusCode> {
    let _ = queries::delete_taskset(&db, taskset_id, user.group_id.unwrap()).await?;

    Ok(())
}
