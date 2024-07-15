use axum::Json;
use axum::{extract::State, http::StatusCode};
use sea_orm::DatabaseConnection;

use crate::{queries::taskset as queries, records::taskset::ResponseTaskSet};

pub async fn all(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseTaskSet>>, StatusCode> {
    let tasksets = queries::get_all_tasksets(&db, None)
        .await?
        .into_iter()
        .map(|elt| elt.into())
        .collect();

    Ok(Json(tasksets))
}
