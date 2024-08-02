use axum::http::StatusCode;
use chrono::{Duration, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DatabaseTransaction, EntityTrait,
    ModelTrait, QueryFilter, Set, TransactionTrait, TryIntoModel,
};
use uuid::Uuid;

use crate::{
    database::{
        groups_invitations::{self, Entity as GroupsInvitations},
        task_assign::{self, Entity as TaskAssign},
        user_groups::{self, Entity as UserGroups, Model as UserGroupsModel},
        vgroup::{self, Entity as Group, Model as GroupModel},
        vuser::{self, Entity as User},
    },
    records::group::InsertGroup,
};

use super::task::get_all_tasks;

pub async fn get_all_groups(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Vec<GroupModel>, StatusCode> {
    let groups = User::find()
        .find_with_related(Group)
        .filter(vuser::Column::Id.eq(user_id))
        .all(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .next()
        .ok_or(StatusCode::NOT_FOUND)?
        .1;

    Ok(groups)
}

pub async fn get_group(
    db: &DatabaseConnection,
    user_id: i32,
    group_id: i32,
) -> Result<GroupModel, StatusCode> {
    let group = Some(
        Group::find()
            .filter(vgroup::Column::Id.eq(group_id))
            .find_with_related(User)
            .all(db)
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

pub async fn generate_invitation_code(
    db: &DatabaseConnection,
    group_id: i32,
) -> Result<String, StatusCode> {
    let invitation_code = Uuid::new_v4().to_string();

    let invitation_row = groups_invitations::ActiveModel {
        invitation_code: Set(invitation_code.clone()),
        vgroup_id: Set(group_id),
        expiration_date: Set((Utc::now() + Duration::days(1)).into()),
        ..Default::default()
    };

    invitation_row
        .save(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(invitation_code)
}

pub async fn accept_invitation(
    db: &DatabaseConnection,
    user_id: i32,
    invitation_code: String,
) -> Result<UserGroupsModel, StatusCode> {
    let txn = db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let invitation = GroupsInvitations::find()
        .filter(groups_invitations::Column::InvitationCode.eq(invitation_code))
        .one(&txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .filter(|invitation| invitation.expiration_date >= Utc::now())
        .ok_or(StatusCode::NOT_ACCEPTABLE)?;

    let user_group_row = user_groups::ActiveModel {
        vuser_id: Set(user_id),
        vgroup_id: Set(invitation.vgroup_id),
        role: Set(crate::database::sea_orm_active_enums::RoleType::Member),
    };

    let user_group_row = user_group_row
        .insert(&txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    invitation
        .delete(&txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(user_group_row)
}

pub async fn leave_group(
    db: &DatabaseConnection,
    user_id: i32,
    group_id: i32,
) -> Result<(), StatusCode> {
    let group_tasks = get_all_tasks(db, None, Some(group_id)).await?;

    let txn = db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = UserGroups::delete_by_id((user_id, group_id))
        .exec(&txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    let _ = TaskAssign::delete_many()
        .filter(
            Condition::all()
                .add(task_assign::Column::TaskId.is_in(group_tasks.iter().map(|v| v.id)))
                .add(task_assign::Column::UserAssign.eq(user_id)),
        )
        .exec(&txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

pub async fn create_group(
    db: &DatabaseConnection,
    user_id: i32,
    group: InsertGroup,
) -> Result<GroupModel, StatusCode> {
    let txn = db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let group = vgroup::ActiveModel {
        name: Set(group.name),
        ..Default::default()
    };

    let group = save_active_group(&txn, group).await?;

    let user_group = user_groups::ActiveModel {
        vgroup_id: Set(group.id),
        vuser_id: Set(user_id),
        role: Set(crate::database::sea_orm_active_enums::RoleType::Member),
    };

    user_group
        .insert(&txn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    txn.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(group)
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
