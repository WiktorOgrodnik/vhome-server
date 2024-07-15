use axum::http::StatusCode;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::database::vuser::{self, Entity as User, Model as UserModel};

pub async fn find_by_username(
    db: &DatabaseConnection,
    username: String,
) -> Result<UserModel, StatusCode> {
    User::find()
        .filter(vuser::Column::Login.eq(username))
        .one(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::BAD_REQUEST)
}

pub async fn find_by_id(db: &DatabaseConnection, user_id: i32) -> Result<UserModel, StatusCode> {
    User::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)
}
