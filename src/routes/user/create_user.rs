use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{IsolationLevel,DatabaseConnection, TransactionTrait};

use crate::{
    queries::user as queries,
    records::user::{RequestUser, ResponseUser},
};

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(user): Json<RequestUser>,
) -> Result<(StatusCode, Json<ResponseUser>), StatusCode> {
    let txn = db
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_exists = queries::get_user_by_username(&txn, &user.username)
        .await
        .is_ok();

    if user_exists {
        return Err(StatusCode::BAD_REQUEST);
    }

    let user = queries::add_user(&txn, user).await?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = ResponseUser {
        id: user.id,
        username: user.login,
        created_at: user.created_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}
