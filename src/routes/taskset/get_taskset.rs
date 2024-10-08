use axum::extract::Path;
use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use crate::records::user::UserExtension;
use crate::{queries::taskset as queries, records::taskset::ResponseTaskSet};

pub async fn get_taskset(
    Extension(user): Extension<UserExtension>,
    Path(taskset_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseTaskSet>, StatusCode> {
    let user = user.force_group_selected()?;

    let taskset = queries::get_taskset_db(&db, taskset_id, user.group_id)
        .await?
        .into();

    Ok(Json(taskset))
}
