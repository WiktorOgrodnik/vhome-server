use axum::{extract::State, http::StatusCode};
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

pub async fn leave_group(
    Extension(user): Extension<UserExtension>,
    State(secret): State<SecretWrapper>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseUserLogin>, StatusCode> {
    let _ = delete_token(&db, user.id, &user.token).await?;

    let token = create_token(&secret.0, Some(user.id), TokenType::Normal, None)?;
    let token = save_token(&db, Some(user.id), &token, TokenType::Normal).await?;

    queries::leave_group(
        &db,
        user.id,
        user.group_id.expect("Expect being in group here!"),
    )
    .await?;

    let response = ResponseUserLogin {
        id: user.id,
        username: user.username,
        token: token.token,
        group: None,
    };

    Ok(Json(response))
}
