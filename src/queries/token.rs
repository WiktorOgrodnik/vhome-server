use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};

use crate::database::tokens::{self, Entity as Token, Model as TokenModel};

pub async fn get_token(
    db: &DatabaseConnection,
    user_id: i32,
    token: &str,
) -> Result<TokenModel, StatusCode> {
    Token::find()
        .filter(
            Condition::all()
                .add(tokens::Column::VuserId.eq(user_id))
                .add(tokens::Column::Token.eq(token)),
        )
        .one(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::BAD_REQUEST)
}

pub async fn save_token(
    db: &DatabaseConnection,
    user_id: i32,
    token: &str,
) -> Result<TokenModel, StatusCode> {
    let token = tokens::ActiveModel {
        vuser_id: Set(user_id),
        token: Set(token.to_owned()),
    };

    token
        .insert(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_token(
    db: &DatabaseConnection,
    user_id: i32,
    token: &str,
) -> Result<sea_orm::DeleteResult, StatusCode> {
    let token = get_token(db, user_id, token).await?.into_active_model();

    Token::delete(token)
        .exec(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
