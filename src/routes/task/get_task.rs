use axum::extract::Path;
use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use crate::records::user::UserExtension;
use crate::{queries::task as queries, records::task::ResponseTask};

pub async fn one(
    Extension(user): Extension<UserExtension>,
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseTask>, StatusCode> {
    let task = queries::get_task(&db, task_id, user.group_id).await?.into();

    Ok(Json(task))
}
