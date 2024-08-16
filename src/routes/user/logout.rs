use axum::{extract::State, http::StatusCode, Extension};
use sea_orm::{IsolationLevel,DatabaseConnection, TransactionTrait};

use crate::{
    queries::token as queries,
    records::user::{GroupUnselectedPayload, UserExtension},
};

pub async fn logout(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
) -> Result<(), StatusCode> {
    let user: GroupUnselectedPayload = user.into();

    let txn = db
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = queries::delete_token(&txn, user.id, &user.token).await?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
