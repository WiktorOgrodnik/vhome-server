use axum::http::StatusCode;
use bcrypt::hash;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DatabaseTransaction, EntityTrait,
    QueryFilter, Set, TryIntoModel,
};

use crate::{
    database::{
        vgroup::{self, Entity as Group},
        vuser::{self, Entity as User, Model as UserModel},
    },
    records::user::RequestUser,
};

pub async fn get_user(txn: &DatabaseTransaction, user_id: i32) -> Result<UserModel, StatusCode> {
    User::find_by_id(user_id)
        .one(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn get_user_db(txn: &DatabaseConnection, user_id: i32) -> Result<UserModel, StatusCode> {
    User::find_by_id(user_id)
        .one(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn get_user_by_username(
    txn: &DatabaseTransaction,
    username: &str,
) -> Result<UserModel, StatusCode> {
    User::find()
        .filter(vuser::Column::Login.eq(username))
        .one(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::BAD_REQUEST)
}

pub async fn get_users(
    txn: &DatabaseTransaction,
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
        .all(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|(user, _)| user)
        .collect::<Vec<_>>())
}

pub async fn get_user_image(
    txn: &DatabaseTransaction,
    user_id: i32,
) -> Result<Option<Vec<u8>>, StatusCode> {
    Ok(get_user(txn, user_id).await?.picutre)
}

pub async fn add_user(
    txn: &DatabaseTransaction,
    user: RequestUser,
) -> Result<UserModel, StatusCode> {
    let active_user = vuser::ActiveModel {
        login: Set(user.username),
        passwd: Set(hash(user.password, 4).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?),
        created_at: Set(Utc::now().into()),
        ..Default::default()
    };

    save_active_user(txn, active_user).await
}

pub async fn save_active_user(
    txn: &DatabaseTransaction,
    user: vuser::ActiveModel,
) -> Result<UserModel, StatusCode> {
    user.save(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
