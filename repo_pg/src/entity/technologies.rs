//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "technologies")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::projects_technologies::Entity")]
    ProjectsTechnologies,
}

impl Related<super::projects_technologies::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProjectsTechnologies.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
