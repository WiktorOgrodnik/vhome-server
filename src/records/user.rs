use crate::database::vuser::Model as UserModel;
use axum::http::StatusCode;
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
    pub group: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    pub id: i32,
    pub username: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum UserExtension {
    GroupUnselected(GroupUnselectedPayload),
    GroupSelected(GroupSelectedPayload),
}

impl UserExtension {
    pub fn force_group_selected(self) -> Result<GroupSelectedPayload, StatusCode> {
        match self {
            Self::GroupUnselected(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            Self::GroupSelected(payload) => Ok(payload),
        }
    }

    pub fn force_group_unselected(self) -> Result<GroupUnselectedPayload, StatusCode> {
        match self {
            Self::GroupUnselected(payload) => Ok(payload),
            Self::GroupSelected(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GroupUnselectedPayload {
    pub id: i32,
    pub username: String,
    pub token: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GroupSelectedPayload {
    pub id: i32,
    pub group_id: i32,
    pub username: String,
    pub token: String,
}

impl From<UserModel> for GroupUnselectedPayload {
    fn from(value: UserModel) -> Self {
        GroupUnselectedPayload {
            id: value.id,
            username: value.login,
            token: "".to_owned(),
        }
    }
}

impl From<GroupSelectedPayload> for GroupUnselectedPayload {
    fn from(value: GroupSelectedPayload) -> Self {
        GroupUnselectedPayload {
            id: value.id,
            username: value.username,
            token: value.token,
        }
    }
}

impl From<UserExtension> for GroupUnselectedPayload {
    fn from(value: UserExtension) -> Self {
        match value {
            UserExtension::GroupUnselected(payload) => payload,
            UserExtension::GroupSelected(payload) => payload.into(),
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
