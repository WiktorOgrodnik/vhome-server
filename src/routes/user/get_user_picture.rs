use axum::extract::Path;
use axum::http::header::CONTENT_TYPE;
use axum::response::{AppendHeaders, IntoResponse};
use axum::{extract::State, http::StatusCode};
use sea_orm::{IsolationLevel,DatabaseConnection, TransactionTrait};

use crate::queries::user as queries;

pub async fn get_user_picture(
    Path(user_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<impl IntoResponse, StatusCode> {
    let txn = db
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let picture = queries::get_user_image(&txn, user_id)
        .await?
        .ok_or(StatusCode::NO_CONTENT)?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if picture.is_empty() {
        Err(StatusCode::NO_CONTENT)
    } else {
        Ok((AppendHeaders([(CONTENT_TYPE, "image/png")]), picture))
    }
}
