use axum::extract::Path;
use axum::http::header::CONTENT_TYPE;
use axum::response::{AppendHeaders, IntoResponse};
use axum::{extract::State, http::StatusCode};
use sea_orm::DatabaseConnection;

use crate::queries::user as queries;

pub async fn get_user_picture(
    Path(user_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<impl IntoResponse, StatusCode> {
    let picture = queries::get_user_image(&db, user_id)
        .await?
        .ok_or(StatusCode::NO_CONTENT)?;

    if picture.is_empty() {
        Err(StatusCode::NO_CONTENT)
    } else {
        Ok((AppendHeaders([(CONTENT_TYPE, "image/png")]), picture))
    }
}
