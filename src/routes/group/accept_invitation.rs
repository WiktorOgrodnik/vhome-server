use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

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
    let invitation = queries::accept_invitation(&db, user.id, body).await?;

    let group = queries::get_group(&db, user.id, invitation.vgroup_id).await?;

    let _ = delete_token(&db, user.id, &user.token).await?;

    let token = create_token(
        &secret.0,
        Some(user.id),
        TokenType::Normal,
        Some(invitation.vgroup_id),
    )?;
    let token = save_token(&db, Some(user.id), &token, TokenType::Normal).await?;

    let response = ResponseUserLogin {
        id: user.id,
        username: user.username,
        token: token.token,
        group: Some(group.name),
    };

    Ok(Json(response))
}
