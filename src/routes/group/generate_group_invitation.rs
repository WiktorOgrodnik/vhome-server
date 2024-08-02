use axum::Extension;
use axum::{extract::State, http::StatusCode};
use sea_orm::DatabaseConnection;

use crate::queries::group as queries;
use crate::records::user::UserExtension;

pub async fn generate_group_invitation(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
) -> Result<(StatusCode, String), StatusCode> {
    let invitation_code =
        queries::generate_invitation_code(&db, user.group_id.expect("Have to be in group here!"))
            .await?;

    Ok((StatusCode::CREATED, invitation_code))
}
