use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use crate::records::user::UserExtension;
use crate::{queries::device as queries, records::device::ResponseDevice};

pub async fn get_devices(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseDevice>>, StatusCode> {
    let devices = queries::get_all_devices(&db, user.group_id)
        .await?
        .into_iter()
        .map(|elt| elt.into())
        .collect();

    Ok(Json(devices))
}
