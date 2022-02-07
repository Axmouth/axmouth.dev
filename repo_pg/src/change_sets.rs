use crate::entity;
use crate::extra::UserRole;
use crate::schema::*;
use axum::headers::Date;
use chrono::{NaiveDateTime, Utc, DateTime};
use sea_orm::ActiveValue::{self, NotSet, Set};
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "blog_post_comments"]
pub struct UpdateBlogPostComment {
    pub body: Option<String>,
    pub updated_at: Option<Option<NaiveDateTime>>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "blog_posts"]
pub struct UpdateBlogPost {
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
    pub updated_at: Option<Option<NaiveDateTime>>,
    pub description: Option<Option<String>>,
    pub slug: Option<String>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "projects"]
pub struct UpdateProject {
    pub body: Option<String>,
    pub updated_at: Option<Option<NaiveDateTime>>,
    pub description: Option<Option<String>>,
    pub cover_image: Option<Option<String>>,
    pub name: Option<String>,
    pub published: Option<bool>,
    pub slug: Option<String>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "refresh_tokens"]
pub struct UpdateRefreshToken {
    pub invalidated: Option<bool>,
    pub used: Option<bool>,
}

impl From<UpdateRefreshToken> for entity::refresh_tokens::ActiveModel {
    fn from(value: UpdateRefreshToken) -> Self {
        entity::refresh_tokens::ActiveModel {
            id: NotSet,
            jwt_id: NotSet,
            user_id: NotSet,
            invalidated: opt_to_av(value.invalidated),
            used: opt_to_av(value.used),
            created_at: NotSet,
            expires_at: NotSet,
        }
    }
}

fn opt_to_av<T>(opt: Option<T>) -> ActiveValue<T>
where
    sea_orm::Value: From<T>,
{
    match opt {
        Some(v) => Set(v),
        None => NotSet,
    }
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "users"]
pub struct UpdateUser {
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub password: Option<String>,
    pub role: Option<UserRole>,
    pub updated_at: Option<Option<NaiveDateTime>>,
}

impl From<UpdateUser> for entity::users::ActiveModel {
    fn from(value: UpdateUser) -> Self {
        let role = match value.role.map(From::from) {
            Some(v) => Set(v),
            None => NotSet,
        };
        entity::users::ActiveModel {
            email: opt_to_av(value.email),
            display_name: opt_to_av(value.display_name),
            password: opt_to_av(value.password),
            role,
            updated_at: opt_to_av(value.updated_at.map(|uu| uu.map(|u| <DateTime<Utc> as From>::from(u)))),
            id: NotSet,
            created_at: NotSet,
        }
    }
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "home_page_links"]
pub struct UpdateHomePageLink {
    pub name: Option<String>,
    pub target: Option<String>,
    pub image: Option<String>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "categories"]
pub struct UpdateCategory {
    pub name: Option<String>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "blog_posts_categories"]
pub struct UpdateBlogPostCategory {
    pub blog_post_id: Option<i32>,
    pub category_id: Option<i32>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "technologies"]
pub struct UpdateTechnology {
    pub name: Option<String>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "text_bodies"]
pub struct UpdateTextBody {
    pub title: Option<Option<String>>,
    pub slug: Option<String>,
    pub body: Option<String>,
    pub url_used: Option<Option<String>>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "verify_email_tokens"]
pub struct UpdateVerifyEmailToken {
    pub invalidated: Option<bool>,
    pub used: Option<bool>,
}

impl From<UpdateVerifyEmailToken> for entity::verify_email_tokens::ActiveModel {
    fn from(value: UpdateVerifyEmailToken) -> Self {
        entity::verify_email_tokens::ActiveModel {
            id: NotSet,
            user_id: NotSet,
            invalidated: opt_to_av(value.invalidated),
            used: opt_to_av(value.used),
            created_at: NotSet,
            expires_at: NotSet,
            token: NotSet,
            email: NotSet,
            old_email: NotSet,
        }
    }
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "blog_post_comment_ratings"]
pub struct UpdateBlogPostCommentRating {
    pub is_like: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[table_name = "change_password_tokens"]
pub struct UpdateChangePasswordToken {
    pub invalidated: Option<bool>,
    pub used: Option<bool>,
}

impl From<UpdateChangePasswordToken> for entity::change_password_tokens::ActiveModel {
    fn from(value: UpdateChangePasswordToken) -> Self {
        entity::change_password_tokens::ActiveModel {
            id: NotSet,
            user_id: NotSet,
            invalidated: opt_to_av(value.invalidated),
            used: opt_to_av(value.used),
            created_at: NotSet,
            expires_at: NotSet,
            token: NotSet,
        }
    }
}
