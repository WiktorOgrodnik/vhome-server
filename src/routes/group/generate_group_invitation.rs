use axum::Extension;
use axum::{extract::State, http::StatusCode};
use sea_orm::{IsolationLevel,DatabaseConnection, TransactionTrait};

use crate::queries::group as queries;
use crate::records::user::UserExtension;
use crate::utilities::invitation_code::generate_invitation_code;

pub async fn generate_group_invitation(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
) -> Result<(StatusCode, String), StatusCode> {
    let user = user.force_group_selected()?;
    let txn = db
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let invitation_code = generate_invitation_code();
    let invitation = queries::add_invitation_code(&txn, invitation_code, user.group_id).await?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, invitation.invitation_code))
}
