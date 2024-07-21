use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use crate::queries::taskset as queries;
use crate::records::taskset::{InsertTaskset, ResponseTaskSet};
use crate::records::user::UserExtension;

pub async fn add_taskset(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
    Json(taskset): Json<InsertTaskset>,
) -> Result<(StatusCode, Json<ResponseTaskSet>), StatusCode> {
    let taskset: ResponseTaskSet = queries::create_taskset(&db, taskset, user.group_id.unwrap())
        .await?
        .into();

    Ok((StatusCode::CREATED, Json(taskset)))
}
