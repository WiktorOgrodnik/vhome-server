use axum::extract::Path;
use axum::Extension;
use axum::{extract::State, http::StatusCode};
use sea_orm::DatabaseConnection;

use crate::queries::task as queries;
use crate::records::user::UserExtension;

pub async fn set_assign(
    Extension(user): Extension<UserExtension>,
    Path((task_id, user_id)): Path<(i32, i32)>,
    State(db): State<DatabaseConnection>,
) -> Result<(), StatusCode> {
    queries::set_assign(&db, task_id, user_id, user.group_id.unwrap()).await
}

pub async fn set_unassing(
    Extension(user): Extension<UserExtension>,
    Path((task_id, user_id)): Path<(i32, i32)>,
    State(db): State<DatabaseConnection>,
) -> Result<(), StatusCode> {
    queries::set_unassign(&db, task_id, user_id, user.group_id.unwrap()).await
}
