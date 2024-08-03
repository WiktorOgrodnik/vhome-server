use axum::body::Bytes;
use axum::Extension;
use axum::{extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel, Set};

use crate::queries::user::find_by_id;
use crate::records::user::UserExtension;

pub async fn add_user_picture(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
    body: Bytes,
) -> Result<(), StatusCode> {
    let mut user_active = find_by_id(&db, user.id).await?.into_active_model();

    user_active.picutre = Set(Some(body.to_vec()));
    user_active
        .save(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
