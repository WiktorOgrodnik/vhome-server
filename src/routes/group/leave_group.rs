use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::{IsolationLevel,DatabaseConnection, TransactionTrait};

use crate::queries::group as queries;
use crate::queries::task as task_queries;
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
    let user = user.force_group_selected()?;

    let txn = db
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = delete_token(&txn, user.id, &user.token).await?;

    let token = create_token(&secret.0, Some(user.id), TokenType::Normal, None)?;
    let token = save_token(&txn, Some(user.id), &token, TokenType::Normal).await?;

    let group_tasks = task_queries::get_all_tasks(&txn, None, Some(user.group_id)).await?;
    let _ = task_queries::delete_user_tasks_assigns(&txn, user.id, &group_tasks).await?;
    let _ = queries::delete_user_group(&txn, user.id, user.group_id).await?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = ResponseUserLogin {
        id: user.id,
        username: user.username,
        token: token.token,
        group: None,
    };

    Ok(Json(response))
}
