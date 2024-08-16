use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use crate::records::user::UserExtension;
use crate::{queries::taskset as queries, records::taskset::ResponseTaskSet};

pub async fn get_tasksets(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseTaskSet>>, StatusCode> {
    let _ = user.force_group_selected()?;
    let tasksets = queries::get_tasksets_db(&db, None)
        .await?
        .into_iter()
        .map(|elt| elt.into())
        .collect();
    Ok(Json(tasksets))
}
