use crate::database::vuser::Model as UserModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    pub id: i32,
    pub username: String,
    pub token: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserExtension {
    pub id: i32,
    pub username: String,
    pub token: String,
    pub group_id: Option<i32>,
}

impl From<UserModel> for UserExtension {
    fn from(value: UserModel) -> Self {
        UserExtension {
            id: value.id,
            username: value.login,
            token: "".to_owned(),
            group_id: None,
        }
    }
}
