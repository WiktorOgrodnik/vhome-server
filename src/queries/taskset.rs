use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};

use crate::{
    database::task::{self, Entity as Task},
    database::taskset::{self, Entity as TaskSet, Model as TaskSetModel},
    records::taskset::InsertTaskset,
};

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

pub async fn create_taskset(
    db: &DatabaseConnection,
    taskset: InsertTaskset,
    group_id: i32,
) -> Result<TaskSetModel, StatusCode> {
    let taskset = taskset::ActiveModel {
        name: Set(taskset.name),
        vgroup_id: Set(group_id),
        ..Default::default()
    };

    save_active_taskset(db, taskset).await
}

pub async fn save_active_taskset(
    db: &DatabaseConnection,
    taskset: taskset::ActiveModel,
) -> Result<TaskSetModel, StatusCode> {
    taskset
        .save(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_taskset(
    db: &DatabaseConnection,
    taskset_id: i32,
    group_id: i32,
) -> Result<sea_orm::DeleteResult, StatusCode> {
    let taskset = get_taskset(db, taskset_id, group_id)
        .await?
        .into_active_model();

    TaskSet::delete(taskset)
        .exec(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Task::delete_many()
        .filter(task::Column::TasksetId.eq(taskset_id))
        .exec(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
