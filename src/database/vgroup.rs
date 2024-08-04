//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "vgroup")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::device::Entity")]
    Device,
    #[sea_orm(has_many = "super::groups_invitations::Entity")]
    GroupsInvitations,
    #[sea_orm(has_many = "super::taskset::Entity")]
    Taskset,
    #[sea_orm(has_many = "super::user_groups::Entity")]
    UserGroups,
}

impl Related<super::device::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Device.def()
    }
}

impl Related<super::groups_invitations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GroupsInvitations.def()
    }
}

impl Related<super::taskset::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Taskset.def()
    }
}

impl Related<super::user_groups::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserGroups.def()
    }
}

impl Related<super::vuser::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_groups::Relation::Vuser.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_groups::Relation::Vgroup.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
