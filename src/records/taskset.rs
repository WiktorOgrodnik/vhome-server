use crate::database::taskset::Model as TaskSetModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseTaskSet {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct InsertTaskset {
    pub name: String,
}

impl From<TaskSetModel> for ResponseTaskSet {
    fn from(value: TaskSetModel) -> Self {
        ResponseTaskSet {
            id: value.id,
            name: value.name,
        }
    }
}
