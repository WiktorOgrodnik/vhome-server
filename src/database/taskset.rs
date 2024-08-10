//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "taskset")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub vgroup_id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::task::Entity")]
    Task,
    #[sea_orm(
        belongs_to = "super::vgroup::Entity",
        from = "Column::VgroupId",
        to = "super::vgroup::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Vgroup,
}

impl Related<super::task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl Related<super::vgroup::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Vgroup.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
