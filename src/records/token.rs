use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::database::sea_orm_active_enums::TokenType as DatabaseTokenType;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    Device,
    Normal,
}

impl Into<DatabaseTokenType> for TokenType {
    fn into(self) -> DatabaseTokenType {
        match self {
            TokenType::Normal => DatabaseTokenType::Normal,
            TokenType::Device => DatabaseTokenType::Device,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub user_id: i32,
    pub token_t: TokenType,
    pub related_id: Option<i32>,
}

impl Claims {
    pub fn force_token_t(self, token_t: TokenType) -> Result<Self, StatusCode> {
        if self.token_t == token_t {
            Ok(self)
        } else {
            Err(StatusCode::BAD_REQUEST)
        }
    }
}
