use axum::body::Bytes;
use axum::Extension;
use axum::{extract::State, http::StatusCode};
use sea_orm::{IsolationLevel,ActiveModelTrait, DatabaseConnection, IntoActiveModel, Set, TransactionTrait};

use crate::queries::user as queries;
use crate::records::user::UserExtension;

pub async fn add_user_picture(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
    body: Bytes,
) -> Result<(), StatusCode> {
    let user = user.force_group_selected()?;

    let txn = db
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut user_active = queries::get_user(&txn, user.id).await?.into_active_model();

    user_active.picutre = Set(Some(body.to_vec()));
    user_active
        .save(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
