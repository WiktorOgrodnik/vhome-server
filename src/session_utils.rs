use serde::{Deserialize, Serialize};

use crate::records::{vgroup, vuser};
use crate::roles::Roles;

#[derive(Debug, Clone)]
pub enum SessionError {
    UserNotLoggedIn,
    GroupNotSelected,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserGroupSessionInd {
    pub group: vgroup::Data,
    pub roles: Vec<Roles>,
}

pub async fn get_user(request: &crate::Request) -> Result<vuser::Data, SessionError> {
    let user: Option<vuser::Data> = request.session().get("user");

    user.ok_or(SessionError::UserNotLoggedIn)
}

pub async fn get_user_group_session_ind(
    request: &crate::Request,
) -> Result<UserGroupSessionInd, SessionError> {
    let _ = get_user(request).await?;

    let group: Option<UserGroupSessionInd> = request.session().get("user_group");

    group.ok_or(SessionError::GroupNotSelected)
}

pub async fn get_group(request: &crate::Request) -> Result<vgroup::Data, SessionError> {
    let group = get_user_group_session_ind(request).await?;

    Ok(group.group)
}

pub async fn get_group_rules(request: &crate::Request) -> Result<Vec<Roles>, SessionError> {
    let group = get_user_group_session_ind(request).await?;

    Ok(group.roles)
}
