use axum::http::StatusCode;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use crate::database::{
    vgroup::{self, Entity as Group, Model as GroupModel},
    vuser::{self, Entity as User, Model as UserModel},
};

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

pub async fn get_users(
    db: &DatabaseConnection,
    group_id: Option<i32>,
) -> Result<Vec<UserModel>, StatusCode> {
    let condition = if let Some(id) = group_id {
        Condition::all().add(vgroup::Column::Id.eq(id))
    } else {
        Condition::all()
    };

    Ok(User::find()
        .find_with_related(Group)
        .filter(condition)
        .all(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|(user, _)| user)
        .collect::<Vec<_>>())
}

pub async fn get_user_image(db: &DatabaseConnection, user_id: i32) -> Result<Vec<u8>, StatusCode> {
    Ok(find_by_id(db, user_id).await?.picutre)
}
