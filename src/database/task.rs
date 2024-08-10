//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub content: String,
    pub completed: bool,
    pub taskset_id: i32,
    pub completed_time: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::task_assign::Entity")]
    TaskAssign,
    #[sea_orm(
        belongs_to = "super::taskset::Entity",
        from = "Column::TasksetId",
        to = "super::taskset::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Taskset,
}

impl Related<super::task_assign::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaskAssign.def()
    }
}

impl Related<super::taskset::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Taskset.def()
    }
}

impl Related<super::vuser::Entity> for Entity {
    fn to() -> RelationDef {
        super::task_assign::Relation::Vuser.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::task_assign::Relation::Task.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
