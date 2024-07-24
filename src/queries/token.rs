use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};

use crate::database::sea_orm_active_enums::TokenType as DatabaseTokenType;
use crate::database::tokens::{self, Entity as Token, Model as TokenModel};
use crate::records::token::TokenType;

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

pub async fn get_normal_token(
    db: &DatabaseConnection,
    user_id: i32,
    token: &str,
) -> Result<TokenModel, StatusCode> {
    Token::find()
        .filter(
            Condition::all()
                .add(tokens::Column::VuserId.eq(user_id))
                .add(tokens::Column::Token.eq(token))
                .add(tokens::Column::TokenT.eq::<DatabaseTokenType>(TokenType::Normal.into())),
        )
        .one(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::BAD_REQUEST)
}

pub async fn get_device_token(
    db: &DatabaseConnection,
    token: &str,
) -> Result<TokenModel, StatusCode> {
    Token::find()
        .filter(
            Condition::all()
                .add(tokens::Column::Token.eq(token))
                .add(tokens::Column::TokenT.eq::<DatabaseTokenType>(TokenType::Device.into())),
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
    token_type: TokenType,
) -> Result<TokenModel, StatusCode> {
    let token = tokens::ActiveModel {
        vuser_id: Set(user_id),
        token: Set(token.to_owned()),
        token_t: Set(token_type.into()),
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
