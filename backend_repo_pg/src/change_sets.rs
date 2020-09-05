use crate::extra::UserRole;
use crate::schema::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "blog_post_comments"]
pub struct UpdateBlogPostComment {
    pub body: Option<String>,
    pub updated_at: Option<Option<NaiveDateTime>>,
}

#[derive(AsChangeset, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "blog_posts"]
pub struct UpdateBlogPost {
    pub body: Option<String>,
    pub published: Option<bool>,
    pub updated_at: Option<Option<NaiveDateTime>>,
    pub description: Option<Option<String>>,
}

#[derive(AsChangeset, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "projects"]
pub struct UpdateProject {
    pub body: Option<String>,
    pub updated_at: Option<Option<NaiveDateTime>>,
    pub description: Option<Option<String>>,
    pub cover_image: Option<Option<String>>,
    pub name: Option<String>,
    pub published: Option<bool>,
}

#[derive(AsChangeset, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "refresh_tokens"]
pub struct UpdateRefreshToken {
    pub invalidated: Option<bool>,
    pub used: Option<bool>,
}

#[derive(AsChangeset, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "users"]
pub struct UpdateUser {
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub password: Option<String>,
    pub role: Option<UserRole>,
    pub updated_at: Option<Option<NaiveDateTime>>,
}

#[derive(AsChangeset, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "home_page_links"]
pub struct UpdateHomePageLink {
    pub name: Option<String>,
    pub target: Option<String>,
    pub image: Option<String>,
}

#[derive(AsChangeset, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "categories"]
pub struct UpdateCategory {
    pub name: Option<String>,
}

#[derive(AsChangeset, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "blog_posts_categories"]
pub struct UpdateBlogPostCategory {
    pub blog_post_id: Option<i32>,
    pub category_id: Option<i32>,
}

#[derive(AsChangeset, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "technologies"]
pub struct UpdateTechnology {
    pub name: Option<String>,
}

#[derive(AsChangeset, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "text_bodies"]
pub struct UpdateTextBody {
    pub title: Option<Option<String>>,
    pub slug: Option<String>,
    pub body: Option<String>,
    pub url_used: Option<Option<String>>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(AsChangeset, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "verify_email_tokens"]
pub struct UpdateVerifyEmailToken {
    pub invalidated: Option<bool>,
    pub used: Option<bool>,
}

#[derive(AsChangeset, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "blog_post_comment_ratings"]
pub struct UpdateBlogPostCommentRating {
    pub is_like: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
}
