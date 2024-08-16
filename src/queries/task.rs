use axum::http::StatusCode;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DatabaseTransaction, EntityTrait,
    IntoActiveModel, ModelTrait, QueryFilter, Set, TryIntoModel,
};

use crate::{
    database::{
        task::{self, Entity as Task, Model as TaskModel},
        task_assign::{self, Entity as TaskAssign, Model as TaskAssignModel},
        taskset::Entity as TaskSet,
    },
    records::task::{EditTask, InsertTask},
};

pub async fn get_all_tasks(
    txn: &DatabaseTransaction,
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
        .all(txn)
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

pub async fn get_all_tasks_db(
    txn: &DatabaseConnection,
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
        .all(txn)
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
    txn: &DatabaseTransaction,
    task_id: i32,
    group_id: Option<i32>,
) -> Result<TaskModel, StatusCode> {
    let task = Some(
        Task::find()
            .filter(task::Column::Id.eq(task_id))
            .find_with_related(TaskSet)
            .all(txn)
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

pub async fn get_task_db(
    txn: &DatabaseConnection,
    task_id: i32,
    group_id: Option<i32>,
) -> Result<TaskModel, StatusCode> {
    let task = Some(
        Task::find()
            .filter(task::Column::Id.eq(task_id))
            .find_with_related(TaskSet)
            .all(txn)
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
    txn: &DatabaseTransaction,
    task_id: i32,
    user_id: i32,
) -> Result<TaskAssignModel, StatusCode> {
    TaskAssign::find_by_id((task_id, user_id))
        .one(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::BAD_REQUEST)
}

pub async fn get_task_assigns(
    txn: &DatabaseTransaction,
    task_id: i32,
    group_id: Option<i32>,
) -> Result<Vec<TaskAssignModel>, StatusCode> {
    let task_assign = get_task(txn, task_id, group_id)
        .await?
        .find_related(TaskAssign)
        .all(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(task_assign)
}

pub async fn get_task_assigns_db(
    txn: &DatabaseConnection,
    task_id: i32,
    group_id: Option<i32>,
) -> Result<Vec<TaskAssignModel>, StatusCode> {
    let task_assign = get_task_db(txn, task_id, group_id)
        .await?
        .find_related(TaskAssign)
        .all(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(task_assign)
}

pub async fn is_task_assigned(
    txn: &DatabaseTransaction,
    task_id: i32,
    user_id: i32,
    group_id: Option<i32>,
) -> Result<bool, StatusCode> {
    let task_assigns = get_task_assigns(txn, task_id, group_id).await?;

    let is_assigned = task_assigns.iter().any(|user| user.user_assign == user_id);

    Ok(is_assigned)
}

pub async fn add_task_assign(
    txn: &DatabaseTransaction,
    task_id: i32,
    user_id: i32,
) -> Result<TaskAssignModel, StatusCode> {
    let task_assign = task_assign::ActiveModel {
        task_id: Set(task_id),
        user_assign: Set(user_id),
        assign_time: Set(Utc::now().into()),
    };

    task_assign
        .insert(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_task_assign(
    txn: &DatabaseTransaction,
    task_id: i32,
    user_id: i32,
) -> Result<sea_orm::DeleteResult, StatusCode> {
    TaskAssign::delete_by_id((task_id, user_id))
        .exec(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn create_task(
    txn: &DatabaseTransaction,
    task: InsertTask,
) -> Result<TaskModel, StatusCode> {
    let task = task::ActiveModel {
        title: Set(task.title),
        content: Set(task.content),
        taskset_id: Set(task.taskset_id),
        completed: Set(false),
        ..Default::default()
    };

    save_active_task(txn, task).await
}

pub async fn patch_task(
    txn: &DatabaseTransaction,
    task: TaskModel,
    edited_task: EditTask,
) -> Result<(), StatusCode> {
    let mut active_task = task.into_active_model();

    active_task.title = Set(edited_task.title);
    active_task.content = Set(edited_task.content);
    active_task.last_update = Set(Utc::now().into());

    let _ = save_active_task(txn, active_task).await?;

    Ok(())
}

pub async fn delete_task(
    txn: &DatabaseTransaction,
    task: TaskModel,
) -> Result<sea_orm::DeleteResult, StatusCode> {
    let task = task.into_active_model();

    Task::delete(task)
        .exec(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_task_by_id(
    txn: &DatabaseTransaction,
    task_id: i32,
) -> Result<sea_orm::DeleteResult, StatusCode> {
    Task::delete_by_id(task_id)
        .exec(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_task_by_taskset_id(
    txn: &DatabaseTransaction,
    taskset_id: i32,
) -> Result<sea_orm::DeleteResult, StatusCode> {
    Task::delete_many()
        .filter(task::Column::TasksetId.eq(taskset_id))
        .exec(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_task_assigns(
    txn: &DatabaseTransaction,
    task_id: i32,
) -> Result<sea_orm::DeleteResult, StatusCode> {
    TaskAssign::delete_many()
        .filter(task_assign::Column::TaskId.eq(task_id))
        .exec(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_user_tasks_assigns(
    txn: &DatabaseTransaction,
    user_id: i32,
    tasks: &[TaskModel],
) -> Result<sea_orm::DeleteResult, StatusCode> {
    TaskAssign::delete_many()
        .filter(
            Condition::all()
                .add(task_assign::Column::TaskId.is_in(tasks.iter().map(|v| v.id)))
                .add(task_assign::Column::UserAssign.eq(user_id)),
        )
        .exec(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn change_completed(
    txn: &DatabaseTransaction,
    task: TaskModel,
    completion: bool,
) -> Result<(), StatusCode> {
    let mut task = task.into_active_model();

    task.completed = Set(completion);
    task.last_update = Set(Utc::now().into());

    let _ = save_active_task(txn, task).await;

    Ok(())
}

pub async fn update_task(
    txn: &DatabaseTransaction,
    task: TaskModel,
) -> Result<TaskModel, StatusCode> {
    let mut task = task.into_active_model();

    task.last_update = Set(Utc::now().into());

    save_active_task(txn, task).await
}

pub async fn save_active_task(
    txn: &DatabaseTransaction,
    task: task::ActiveModel,
) -> Result<TaskModel, StatusCode> {
    task.save(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn save_active_task_db(
    txn: &DatabaseConnection,
    task: task::ActiveModel,
) -> Result<TaskModel, StatusCode> {
    task.save(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
