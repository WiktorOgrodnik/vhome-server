use axum::http::StatusCode;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    ModelTrait, QueryFilter, Set, TryIntoModel,
};

use crate::{
    database::{
        task::{self, Entity as Task, Model as TaskModel},
        task_assign::{Entity as TaskAssign, Model as TaskAssignModel},
        taskset::Entity as TaskSet,
    },
    queries::taskset::has_permission,
    records::task::InsertTask,
};

pub async fn get_all_tasks(
    db: &DatabaseConnection,
    taskset_id: Option<i32>,
    group_id: Option<i32>,
) -> Result<Vec<TaskModel>, StatusCode> {
    let condition = if let Some(id) = taskset_id {
        Condition::all().add(task::Column::TasksetId.eq(id))
    } else {
        Condition::all()
    };

    let tasks: Vec<TaskModel> = Task::find()
        .filter(condition.clone())
        .find_with_related(TaskSet)
        .all(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .filter(|(_, tasksets)| {
            group_id.is_none()
                || tasksets
                    .iter()
                    .all(|taskset| Some(taskset.vgroup_id) == group_id)
        })
        .map(|(task, _)| task)
        .collect();

    Ok(tasks)
}

pub async fn get_task(
    db: &DatabaseConnection,
    task_id: i32,
    group_id: Option<i32>,
) -> Result<TaskModel, StatusCode> {
    let task = Some(
        Task::find()
            .filter(task::Column::Id.eq(task_id))
            .find_with_related(TaskSet)
            .all(db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .into_iter()
            .next()
            .ok_or(StatusCode::NOT_FOUND)?,
    )
    .filter(|(_, taskset)| {
        group_id.is_none()
            || taskset
                .iter()
                .all(|taskset| Some(taskset.vgroup_id) == group_id)
    })
    .ok_or(StatusCode::FORBIDDEN)?
    .0;

    Ok(task)
}

pub async fn get_task_assign(
    db: &DatabaseConnection,
    task_id: i32,
    group_id: Option<i32>,
) -> Result<Vec<TaskAssignModel>, StatusCode> {
    let task_assign = get_task(db, task_id, group_id)
        .await?
        .find_related(TaskAssign)
        .all(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(task_assign)
}

pub async fn create_task(
    db: &DatabaseConnection,
    task: InsertTask,
    group_id: i32,
) -> Result<TaskModel, StatusCode> {
    has_permission(db, task.taskset_id, group_id).await?;

    let task = task::ActiveModel {
        title: Set(task.title),
        content: Set(task.content),
        taskset_id: Set(task.taskset_id),
        completed: Set(false),
        ..Default::default()
    };

    save_active_task(db, task).await
}

pub async fn delete_task(
    db: &DatabaseConnection,
    task_id: i32,
    group_id: i32,
) -> Result<sea_orm::DeleteResult, StatusCode> {
    let task = get_task(db, task_id, Some(group_id))
        .await?
        .into_active_model();

    Task::delete(task)
        .exec(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn change_completed(
    db: &DatabaseConnection,
    task_id: i32,
    group_id: i32,
    completion: bool,
) -> Result<(), StatusCode> {
    let mut task: task::ActiveModel = get_task(db, task_id, Some(group_id))
        .await?
        .into_active_model();

    task.completed = Set(completion);
    task.completed_time = Set(match completion {
        true => Some(Utc::now().into()),
        false => None,
    });

    let _ = save_active_task(db, task).await;

    Ok(())
}

pub async fn save_active_task(
    db: &DatabaseConnection,
    task: task::ActiveModel,
) -> Result<TaskModel, StatusCode> {
    task.save(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
