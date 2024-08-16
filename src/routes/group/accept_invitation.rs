use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use sea_orm::{IsolationLevel,DatabaseConnection, TransactionTrait};

use crate::queries::group as queries;
use crate::queries::token::{delete_token, save_token};
use crate::records::{
    token::TokenType,
    user::{ResponseUserLogin, UserExtension},
};
use crate::state::SecretWrapper;
use crate::utilities::token::create_token;

pub async fn accept_invitation(
    Extension(user): Extension<UserExtension>,
    State(secret): State<SecretWrapper>,
    State(db): State<DatabaseConnection>,
    body: String,
) -> Result<Json<ResponseUserLogin>, StatusCode> {
    let user = user.force_group_unselected()?;

    let txn = db
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let invitation = queries::get_valid_invitation_code(&txn, body).await?;
    let group_id = invitation.vgroup_id;
    let _ = queries::delete_invitation_code(&txn, invitation).await?;

    let is_in_group = queries::is_user_in_group(&txn, user.id, group_id).await?;

    if is_in_group {
        return Err(StatusCode::BAD_REQUEST);
    }

    let _ = queries::add_user_group(
        &txn,
        user.id,
        group_id,
        crate::database::sea_orm_active_enums::RoleType::Member,
    )
    .await?;

    let _ = delete_token(&txn, user.id, &user.token).await?;
    let token = create_token(&secret.0, Some(user.id), TokenType::Normal, Some(group_id))?;
    let token = save_token(&txn, Some(user.id), &token, TokenType::Normal).await?;

    let group = queries::get_group(&txn, user.id, group_id).await?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = ResponseUserLogin {
        id: user.id,
        username: user.username,
        token: token.token,
        group: Some(group.name),
    };

    Ok(Json(response))
}
