use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use crate::records::user::UserExtension;
use crate::{queries::group as queries, records::group::ResponseGroup};

pub async fn get_groups(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseGroup>>, StatusCode> {
    let groups = queries::get_all_groups(&db, user.id)
        .await?
        .into_iter()
        .map(|elt| elt.into())
        .collect();

    Ok(Json(groups))
}
