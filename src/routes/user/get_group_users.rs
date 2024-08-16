use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::{IsolationLevel,DatabaseConnection, TransactionTrait};

use crate::records::user::UserExtension;
use crate::{queries::user as queries, records::user::ResponseUser};

pub async fn get_group_users(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseUser>>, StatusCode> {
    let user = user.force_group_selected()?;
    let txn = db
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let users = queries::get_users(&txn, Some(user.group_id))
        .await?
        .into_iter()
        .map(|elt| elt.into())
        .collect();

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}
