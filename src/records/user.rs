use crate::database::vuser::Model as UserModel;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUserLogin {
    pub id: i32,
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    pub id: i32,
    pub username: String,
    pub created_at: DateTimeWithTimeZone,
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

impl From<UserModel> for ResponseUser {
    fn from(value: UserModel) -> Self {
        ResponseUser {
            id: value.id,
            username: value.login,
            created_at: value.created_at,
        }
    }
}
