use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use crate::records::user::UserExtension;
use crate::{queries::taskset as queries, records::taskset::ResponseTaskSet};

pub async fn all(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseTaskSet>>, StatusCode> {
    let tasksets = queries::get_all_tasksets(&db, user.group_id)
        .await?
        .into_iter()
        .map(|elt| elt.into())
        .collect();

    Ok(Json(tasksets))
}
