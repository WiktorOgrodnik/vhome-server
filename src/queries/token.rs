use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DatabaseTransaction, EntityTrait,
    IntoActiveModel, QueryFilter, Set, TryIntoModel,
};

use crate::database::sea_orm_active_enums::TokenType as DatabaseTokenType;
use crate::database::tokens::{self, Entity as Token, Model as TokenModel};
use crate::records::token::TokenType;

pub async fn get_token(
    txn: &DatabaseTransaction,
    user_id: i32,
    token: &str,
) -> Result<TokenModel, StatusCode> {
    Token::find()
        .filter(
            Condition::all()
                .add(tokens::Column::VuserId.eq(user_id))
                .add(tokens::Column::Token.eq(token)),
        )
        .one(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::BAD_REQUEST)
}

pub async fn get_token_by_id(
    txn: &DatabaseTransaction,
    token_id: i32,
) -> Result<TokenModel, StatusCode> {
    Token::find_by_id(token_id)
        .one(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::BAD_REQUEST)
}

pub async fn get_normal_token(
    txn: &DatabaseTransaction,
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
        .one(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::BAD_REQUEST)
}

pub async fn get_normal_token_db(
    txn: &DatabaseConnection,
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
        .one(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::BAD_REQUEST)
}

pub async fn get_device_token(
    txn: &DatabaseTransaction,
    token: &str,
) -> Result<TokenModel, StatusCode> {
    Token::find()
        .filter(
            Condition::all()
                .add(tokens::Column::Token.eq(token))
                .add(tokens::Column::TokenT.eq::<DatabaseTokenType>(TokenType::Device.into())),
        )
        .one(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::BAD_REQUEST)
}

pub async fn get_display_token(
    txn: &DatabaseTransaction,
    user_id: i32,
    token: &str,
) -> Result<TokenModel, StatusCode> {
    Token::find()
        .filter(
            Condition::all()
                .add(tokens::Column::VuserId.eq(user_id))
                .add(tokens::Column::Token.eq(token))
                .add(tokens::Column::TokenT.eq::<DatabaseTokenType>(TokenType::Display.into())),
        )
        .one(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::BAD_REQUEST)
}

pub async fn save_token(
    txn: &DatabaseTransaction,
    user_id: Option<i32>,
    token: &str,
    token_type: TokenType,
) -> Result<TokenModel, StatusCode> {
    let token = tokens::ActiveModel {
        vuser_id: Set(user_id),
        token: Set(token.to_owned()),
        token_t: Set(token_type.into()),
        ..Default::default()
    };

    token
        .save(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_token(
    txn: &DatabaseTransaction,
    user_id: i32,
    token: &str,
) -> Result<sea_orm::DeleteResult, StatusCode> {
    let token = get_token(txn, user_id, token).await?.into_active_model();

    Token::delete(token)
        .exec(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
