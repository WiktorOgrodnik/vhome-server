use axum::{extract::State, http::StatusCode, Extension};
use sea_orm::DatabaseConnection;

use crate::{queries::token as queries, records::user::UserExtension};

pub async fn logout(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
) -> Result<(), StatusCode> {
    let _ = queries::delete_token(&db, user.id, &user.token).await?;

    Ok(())
}
