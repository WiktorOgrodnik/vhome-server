use serde::{Deserialize, Serialize};

use crate::database::task::Model as TaskModel;

#[derive(Serialize, Deserialize)]
pub struct ResponseTask {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub completed: bool,
    pub taskset_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseTaskAssign {
    pub id: i32,
    pub users_id: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct InsertTask {
    pub title: String,
    pub content: String,
    pub taskset_id: i32,
}

impl From<TaskModel> for ResponseTask {
    fn from(value: TaskModel) -> Self {
        ResponseTask {
            id: value.id,
            title: value.title,
            content: value.content,
            completed: value.completed,
            taskset_id: value.taskset_id,
        }
    }
}
