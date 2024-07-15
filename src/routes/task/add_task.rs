use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

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
    let task: ResponseTask = queries::create_task(&db, task, user.group_id.unwrap())
        .await?
        .into();

    Ok((StatusCode::CREATED, Json(task)))
}
