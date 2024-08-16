use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::{IsolationLevel,DatabaseConnection, TransactionTrait};

use crate::records::user::{GroupUnselectedPayload, UserExtension};
use crate::{queries::group as queries, records::group::ResponseGroup};

pub async fn get_groups(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseGroup>>, StatusCode> {
    let user: GroupUnselectedPayload = user.into();
    let txn = db
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let groups = queries::get_groups(&txn, user.id)
        .await?
        .into_iter()
        .map(|elt| elt.into())
        .collect();

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(groups))
}
