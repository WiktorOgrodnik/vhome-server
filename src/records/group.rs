use crate::database::vgroup::Model as GroupModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseGroup {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct InsertGroup {
    pub name: String,
}

impl From<GroupModel> for ResponseGroup {
    fn from(value: GroupModel) -> Self {
        ResponseGroup {
            id: value.id,
            name: value.name,
        }
    }
}
