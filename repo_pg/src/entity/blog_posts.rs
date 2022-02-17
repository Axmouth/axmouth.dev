//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "blog_posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub body: String,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub published: bool,
    pub author_id: i32,
    pub description: Option<String>,
    pub slug: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::AuthorId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
    #[sea_orm(has_many = "super::blog_post_comments::Entity")]
    BlogPostComments,
    #[sea_orm(has_many = "super::blog_posts_categories::Entity")]
    BlogPostsCategories,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::blog_post_comments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BlogPostComments.def()
    }
}

impl Related<super::blog_posts_categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BlogPostsCategories.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}