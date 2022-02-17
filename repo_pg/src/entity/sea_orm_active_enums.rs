//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "admin_log_action")]
pub enum AdminLogAction {
    #[sea_orm(string_value = "Create")]
    Create,
    #[sea_orm(string_value = "Delete")]
    Delete,
    #[sea_orm(string_value = "Update")]
    Update,
}
#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "search_item_type")]
pub enum SearchItemType {
    #[sea_orm(string_value = "Blog Post")]
    BlogPost,
    #[sea_orm(string_value = "External Link")]
    ExternalLink,
    #[sea_orm(string_value = "Page")]
    Page,
    #[sea_orm(string_value = "Project")]
    Project,
}
#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_role")]
pub enum UserRole {
    #[sea_orm(string_value = "Admin")]
    Admin,
    #[sea_orm(string_value = "Ghost")]
    Ghost,
    #[sea_orm(string_value = "Moderator")]
    Moderator,
    #[sea_orm(string_value = "User")]
    User,
}
