use axum::extract::Path;
use axum::Json;
use axum::{extract::State, http::StatusCode};
use sea_orm::{IsolationLevel,DatabaseConnection, TransactionTrait};

use crate::{queries::user as queries, records::user::ResponseUser};

pub async fn get_users(
    Path(user_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let txn = db
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = queries::get_user(&txn, user_id).await?.into();

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}
