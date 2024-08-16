use axum::http::StatusCode;
use chrono::{Duration, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait,
    ModelTrait, QueryFilter, Set, TryIntoModel,
};

use crate::{
    database::{
        groups_invitations::{self, Entity as GroupsInvitations, Model as GroupInviationModel},
        sea_orm_active_enums::RoleType,
        user_groups::{self, Entity as UserGroups, Model as UserGroupsModel},
        vgroup::{self, Entity as Group, Model as GroupModel},
        vuser::{self, Entity as User},
    },
    records::group::InsertGroup,
};

pub async fn get_group(
    txn: &DatabaseTransaction,
    user_id: i32,
    group_id: i32,
) -> Result<GroupModel, StatusCode> {
    let group = Some(
        Group::find()
            .filter(vgroup::Column::Id.eq(group_id))
            .find_with_related(User)
            .all(txn)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .into_iter()
            .next()
            .ok_or(StatusCode::NOT_FOUND)?,
    )
    .filter(|(_, users)| users.iter().any(|user| user.id == user_id))
    .ok_or(StatusCode::FORBIDDEN)?
    .0;

    Ok(group)
}

pub async fn get_group_db(
    txn: &DatabaseConnection,
    user_id: i32,
    group_id: i32,
) -> Result<GroupModel, StatusCode> {
    let group = Some(
        Group::find()
            .filter(vgroup::Column::Id.eq(group_id))
            .find_with_related(User)
            .all(txn)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .into_iter()
            .next()
            .ok_or(StatusCode::NOT_FOUND)?,
    )
    .filter(|(_, users)| users.iter().any(|user| user.id == user_id))
    .ok_or(StatusCode::FORBIDDEN)?
    .0;

    Ok(group)
}

pub async fn get_groups(
    txn: &DatabaseTransaction,
    user_id: i32,
) -> Result<Vec<GroupModel>, StatusCode> {
    let groups = User::find()
        .find_with_related(Group)
        .filter(vuser::Column::Id.eq(user_id))
        .all(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .next()
        .ok_or(StatusCode::NOT_FOUND)?
        .1;

    Ok(groups)
}

pub async fn add_group(
    txn: &DatabaseTransaction,
    group: InsertGroup,
) -> Result<GroupModel, StatusCode> {
    let group = vgroup::ActiveModel {
        name: Set(group.name),
        ..Default::default()
    };

    save_active_group(txn, group).await
}

pub async fn save_active_group(
    txn: &DatabaseTransaction,
    group: vgroup::ActiveModel,
) -> Result<GroupModel, StatusCode> {
    group
        .save(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_valid_invitation_code(
    txn: &DatabaseTransaction,
    invitation_code: String,
) -> Result<GroupInviationModel, StatusCode> {
    GroupsInvitations::find()
        .filter(groups_invitations::Column::InvitationCode.eq(invitation_code))
        .one(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .filter(|invitation| invitation.expiration_date >= Utc::now())
        .ok_or(StatusCode::NOT_ACCEPTABLE)
}

pub async fn delete_invitation_code(
    txn: &DatabaseTransaction,
    invitation_code: GroupInviationModel,
) -> Result<sea_orm::DeleteResult, StatusCode> {
    invitation_code
        .delete(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn add_invitation_code(
    txn: &DatabaseTransaction,
    invitation_code: String,
    group_id: i32,
) -> Result<GroupInviationModel, StatusCode> {
    let invitation_code = groups_invitations::ActiveModel {
        invitation_code: Set(invitation_code.clone()),
        vgroup_id: Set(group_id),
        expiration_date: Set((Utc::now() + Duration::days(1)).into()),
        ..Default::default()
    };

    save_active_invitation_code(txn, invitation_code).await
}

pub async fn save_active_invitation_code(
    txn: &DatabaseTransaction,
    invitation_code: groups_invitations::ActiveModel,
) -> Result<GroupInviationModel, StatusCode> {
    invitation_code
        .save(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn is_user_in_group(
    txn: &DatabaseTransaction,
    user_id: i32,
    group_id: i32,
) -> Result<bool, StatusCode> {
    let user_groups = get_groups(txn, user_id).await?;

    let is_in_group = user_groups.iter().any(|group| group.id == group_id);

    Ok(is_in_group)
}

pub async fn add_user_group(
    txn: &DatabaseTransaction,
    user_id: i32,
    group_id: i32,
    role: RoleType,
) -> Result<UserGroupsModel, StatusCode> {
    let user_group_row = user_groups::ActiveModel {
        vuser_id: Set(user_id),
        vgroup_id: Set(group_id),
        role: Set(role),
    };

    user_group_row
        .insert(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_user_group(
    txn: &DatabaseTransaction,
    user_id: i32,
    group_id: i32,
) -> Result<sea_orm::DeleteResult, StatusCode> {
    UserGroups::delete_by_id((user_id, group_id))
        .exec(txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
