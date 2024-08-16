use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use sea_orm::{IsolationLevel,DatabaseConnection, TransactionTrait};

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
    let user = user.force_group_unselected()?;

    let txn = db
        .begin_with_config(Some(IsolationLevel::Serializable), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let group = queries::add_group(&txn, group).await?;
    let _ = queries::add_user_group(
        &txn,
        user.id,
        group.id,
        crate::database::sea_orm_active_enums::RoleType::Member,
    )
    .await?;

    let _ = delete_token(&txn, user.id, &user.token).await?;

    let token = create_token(&secret.0, Some(user.id), TokenType::Normal, Some(group.id))?;
    let token = save_token(&txn, Some(user.id), &token, TokenType::Normal).await?;

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
