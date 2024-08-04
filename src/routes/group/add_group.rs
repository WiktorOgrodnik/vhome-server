use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use crate::queries::group as queries;
use crate::queries::token::{delete_token, save_token};
use crate::records::group::InsertGroup;
use crate::records::{
    token::TokenType,
    user::{ResponseUserLogin, UserExtension},
};
use crate::state::SecretWrapper;
use crate::utilities::token::create_token;

pub async fn add_group(
    Extension(user): Extension<UserExtension>,
    State(secret): State<SecretWrapper>,
    State(db): State<DatabaseConnection>,
    Json(group): Json<InsertGroup>,
) -> Result<Json<ResponseUserLogin>, StatusCode> {
    let group = queries::create_group(&db, user.id, group).await?;

    let _ = delete_token(&db, user.id, &user.token).await?;

    let token = create_token(&secret.0, user.id, TokenType::Normal, Some(group.id))?;
    let token = save_token(&db, user.id, &token, TokenType::Normal).await?;

    let response = ResponseUserLogin {
        id: user.id,
        username: user.username,
        token: token.token,
        group: Some(group.name),
    };

    Ok(Json(response))
}
