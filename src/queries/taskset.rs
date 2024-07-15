use axum::http::StatusCode;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use crate::database::taskset::{self, Entity as TaskSet, Model as TaskSetModel};

pub async fn has_permission(
    db: &DatabaseConnection,
    taskset_id: i32,
    group_id: i32,
) -> Result<(), StatusCode> {
    let _ = TaskSet::find()
        .filter(taskset::Column::Id.eq(taskset_id))
        .one(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .filter(|taskset| taskset.vgroup_id == group_id)
        .ok_or(StatusCode::FORBIDDEN)?;

    Ok(())
}

pub async fn get_all_tasksets(
    db: &DatabaseConnection,
    group_id: Option<i32>,
) -> Result<Vec<TaskSetModel>, StatusCode> {
    let condition = if let Some(id) = group_id {
        Condition::all().add(taskset::Column::VgroupId.eq(id))
    } else {
        Condition::all()
    };

    TaskSet::find()
        .filter(condition)
        .all(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_taskset(
    db: &DatabaseConnection,
    taskset_id: i32,
    group_id: i32,
) -> Result<TaskSetModel, StatusCode> {
    let taskset = TaskSet::find()
        .filter(
            Condition::all()
                .add(taskset::Column::Id.eq(taskset_id))
                .add(taskset::Column::VgroupId.eq(group_id)),
        )
        .one(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match taskset {
        Some(taskset) => Ok(taskset),
        None => Err(StatusCode::NOT_FOUND),
    }
}
