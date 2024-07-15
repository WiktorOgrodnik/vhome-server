use axum::http::StatusCode;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::database::{
    vgroup::{self, Entity as Group, Model as GroupModel},
    vuser::{self, Entity as User},
};

pub async fn get_all_groups(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Vec<GroupModel>, StatusCode> {
    let groups = User::find()
        .find_with_related(Group)
        .filter(vuser::Column::Id.eq(user_id))
        .all(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .next()
        .ok_or(StatusCode::NOT_FOUND)?
        .1;

    Ok(groups)
}

pub async fn get_group(
    db: &DatabaseConnection,
    user_id: i32,
    group_id: i32,
) -> Result<GroupModel, StatusCode> {
    let group = Some(
        Group::find()
            .filter(vgroup::Column::Id.eq(group_id))
            .find_with_related(User)
            .all(db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .into_iter()
            .next()
            .ok_or(StatusCode::NOT_FOUND)?,
    )
    .filter(|(_, users)| users.iter().all(|user| user.id == user_id))
    .ok_or(StatusCode::FORBIDDEN)?
    .0;

    Ok(group)
}
