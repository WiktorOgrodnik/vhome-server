use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use crate::records::user::UserExtension;
use crate::{queries::user as queries, records::user::ResponseUser};

pub async fn get_group_users(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseUser>>, StatusCode> {
    let users = queries::get_users(&db, user.group_id)
        .await?
        .into_iter()
        .map(|elt| elt.into())
        .collect();

    Ok(Json(users))
}
