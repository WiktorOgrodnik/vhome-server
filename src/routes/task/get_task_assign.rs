use axum::extract::Path;
use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use crate::queries::task as queries;
use crate::records::task::ResponseTaskAssign;
use crate::records::user::UserExtension;

pub async fn get_task_assign(
    Extension(user): Extension<UserExtension>,
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseTaskAssign>, StatusCode> {
    let user_assigns = queries::get_task_assign(&db, task_id, user.group_id)
        .await?
        .iter()
        .map(|assing_model| assing_model.user_assign)
        .collect::<Vec<_>>();

    Ok(Json(ResponseTaskAssign {
        id: task_id,
        users_id: user_assigns,
    }))
}
