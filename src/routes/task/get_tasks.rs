use axum::extract::Path;
use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use crate::records::user::UserExtension;
use crate::{queries::task as queries, records::task::ResponseTask};

pub async fn all(
    Extension(user): Extension<UserExtension>,
    Path(taskset_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let tasks = queries::get_all_tasks(&db, Some(taskset_id), user.group_id)
        .await?
        .into_iter()
        .map(|elt| elt.into())
        .collect();

    Ok(Json(tasks))
}
