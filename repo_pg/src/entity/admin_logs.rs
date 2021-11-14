//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use super::sea_orm_active_enums::AdminLogAction;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "admin_logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub object_id: String,
    pub user_id: i32,
    pub label: String,
    pub model: String,
    pub action_time: DateTime,
    pub action: AdminLogAction,
    pub new_data: Option<String>,
    pub old_data: Option<String>,
    pub base_link: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
