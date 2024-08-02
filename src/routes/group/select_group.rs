use axum::extract::Path;
use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use crate::queries::group as queries;
use crate::queries::token::{delete_token, save_token};
use crate::records::user::ResponseUserLogin;
use crate::records::{token::TokenType, user::UserExtension};
use crate::state::SecretWrapper;
use crate::utilities::token::create_token;

pub async fn select_group(
    Extension(user): Extension<UserExtension>,
    Path(group_id): Path<i32>,
    State(secret): State<SecretWrapper>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseUserLogin>, StatusCode> {
    let group = queries::get_group(&db, user.id, group_id).await?;

    let _ = delete_token(&db, user.id, &user.token).await?;

    let token = create_token(&secret.0, user.id, TokenType::Normal, Some(group_id))?;
    let token = save_token(&db, user.id, &token, TokenType::Normal).await?;

    let response = ResponseUserLogin {
        id: user.id,
        username: user.username,
        token: token.token,
        group: Some(group.name),
    };

    Ok(Json(response))
}

pub async fn unselect_group(
    Extension(user): Extension<UserExtension>,
    State(secret): State<SecretWrapper>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseUserLogin>, StatusCode> {
    let _ = delete_token(&db, user.id, &user.token).await?;

    let token = create_token(&secret.0, user.id, TokenType::Normal, None)?;
    let token = save_token(&db, user.id, &token, TokenType::Normal).await?;

    let response = ResponseUserLogin {
        id: user.id,
        username: user.username,
        token: token.token,
        group: None,
    };

    Ok(Json(response))
}
