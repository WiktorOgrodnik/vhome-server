use axum::extract::Path;
use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use crate::queries::group as queries;
use crate::queries::token::{delete_token, save_token};
use crate::records::user::{ResponseUser, UserExtension};
use crate::state::SecretWrapper;
use crate::utilities::token::create_token;

pub async fn select_group(
    Extension(user): Extension<UserExtension>,
    Path(group_id): Path<i32>,
    State(secret): State<SecretWrapper>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let _ = queries::get_group(&db, user.id, group_id).await?;
    let _ = delete_token(&db, user.id, &user.token).await?;

    let token = create_token(&secret.0, user.id, Some(group_id))?;
    let token = save_token(&db, user.id, &token).await?;

    let response = ResponseUser {
        id: user.id,
        username: user.username,
        token: token.token,
    };

    Ok(Json(response))
}

pub async fn unselect_group(
    Extension(user): Extension<UserExtension>,
    State(secret): State<SecretWrapper>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let _ = delete_token(&db, user.id, &user.token).await?;

    let token = create_token(&secret.0, user.id, None)?;
    let token = save_token(&db, user.id, &token).await?;

    let response = ResponseUser {
        id: user.id,
        username: user.username,
        token: token.token,
    };

    Ok(Json(response))
}
