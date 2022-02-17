//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "blog_post_comment_ratings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub is_like: bool,
    pub user_id: i32,
    pub blog_post_comment_id: i32,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::blog_post_comments::Entity",
        from = "Column::BlogPostCommentId",
        to = "super::blog_post_comments::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    BlogPostComments,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
}

impl Related<super::blog_post_comments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BlogPostComments.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
