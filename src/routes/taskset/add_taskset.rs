use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::{DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::queries::taskset as queries;
use crate::records::taskset::{InsertTaskset, ResponseTaskSet};
use crate::records::user::UserExtension;

pub async fn add_taskset(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
    Json(taskset): Json<InsertTaskset>,
) -> Result<(StatusCode, Json<ResponseTaskSet>), StatusCode> {
    let user = user.force_group_selected()?;
    let txn = db
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let taskset: ResponseTaskSet = queries::add_taskset(&txn, taskset, user.group_id)
        .await?
        .into();

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(taskset)))
}
