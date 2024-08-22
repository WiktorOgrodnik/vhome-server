use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DatabaseTransaction, EntityTrait,
    IntoActiveModel, QueryFilter, Set,
};

use crate::{
    database::taskset::{self, Entity as TaskSet, Model as TaskSetModel},
    records::taskset::InsertTaskset,
};

pub async fn has_permission(
    txn: &DatabaseTransaction,
    taskset_id: i32,
    group_id: i32,
) -> Result<(), StatusCode> {
    let _ = TaskSet::find()
        .filter(taskset::Column::Id.eq(taskset_id))
        .one(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .filter(|taskset| taskset.vgroup_id == group_id)
        .ok_or(StatusCode::FORBIDDEN)?;

    Ok(())
}

pub async fn get_taskset(
    txn: &DatabaseTransaction,
    taskset_id: i32,
    group_id: i32,
) -> Result<TaskSetModel, StatusCode> {
    let taskset = TaskSet::find()
        .filter(
            Condition::all()
                .add(taskset::Column::Id.eq(taskset_id))
                .add(taskset::Column::VgroupId.eq(group_id)),
        )
        .one(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match taskset {
        Some(taskset) => Ok(taskset),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_taskset_db(
    txn: &DatabaseConnection,
    taskset_id: i32,
    group_id: i32,
) -> Result<TaskSetModel, StatusCode> {
    let taskset = TaskSet::find()
        .filter(
            Condition::all()
                .add(taskset::Column::Id.eq(taskset_id))
                .add(taskset::Column::VgroupId.eq(group_id)),
        )
        .one(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match taskset {
        Some(taskset) => Ok(taskset),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_tasksets(
    txn: &DatabaseTransaction,
    group_id: Option<i32>,
) -> Result<Vec<TaskSetModel>, StatusCode> {
    let condition = if let Some(id) = group_id {
        Condition::all().add(taskset::Column::VgroupId.eq(id))
    } else {
        Condition::all()
    };

    TaskSet::find()
        .filter(condition)
        .all(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_tasksets_db(
    txn: &DatabaseConnection,
    group_id: Option<i32>,
) -> Result<Vec<TaskSetModel>, StatusCode> {
    let condition = if let Some(id) = group_id {
        Condition::all().add(taskset::Column::VgroupId.eq(id))
    } else {
        Condition::all()
    };

    TaskSet::find()
        .filter(condition)
        .all(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn add_taskset(
    txn: &DatabaseTransaction,
    taskset: InsertTaskset,
    group_id: i32,
) -> Result<TaskSetModel, StatusCode> {
    let taskset = taskset::ActiveModel {
        name: Set(taskset.name),
        vgroup_id: Set(group_id),
        ..Default::default()
    };

    save_active_taskset(txn, taskset).await
}

pub async fn patch_taskset(
    txn: &DatabaseTransaction,
    taskset: TaskSetModel,
    edited_taskset: InsertTaskset,
) -> Result<TaskSetModel, StatusCode> {
    let mut active_taskset = taskset.into_active_model();

    active_taskset.name = Set(edited_taskset.name);

    save_active_taskset(txn, active_taskset).await
}

pub async fn save_active_taskset(
    txn: &DatabaseTransaction,
    taskset: taskset::ActiveModel,
) -> Result<TaskSetModel, StatusCode> {
    taskset
        .save(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_taskset(
    txn: &DatabaseTransaction,
    taskset_id: i32,
    group_id: i32,
) -> Result<sea_orm::DeleteResult, StatusCode> {
    let _ = get_taskset(txn, taskset_id, group_id).await?;

    TaskSet::delete_by_id(taskset_id)
        .exec(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
