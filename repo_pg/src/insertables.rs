use crate::entity;
use crate::extra::{AdminLogAction, UserRole};
use crate::schema::*;
use sea_orm::ActiveValue;
use serde::Serialize;
use ActiveValue::{NotSet, Set};

use chrono::NaiveDateTime;

#[derive(Insertable, Clone, Serialize)]
#[table_name = "blog_post_comments"]
pub struct NewBlogPostComment {
    pub body: Option<String>,
    pub author_id: i32,
    pub post_id: i32,
}

#[derive(Insertable, Clone, Serialize)]
#[table_name = "blog_posts"]
pub struct NewBlogPost {
    pub title: String,
    pub body: String,
    pub published: bool,
    pub author_id: i32,
    pub description: Option<String>,
    pub slug: String,
}

#[derive(Insertable, Clone, Serialize)]
#[table_name = "projects"]
pub struct NewProject {
    pub body: String,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub name: String,
    pub slug: String,
}

#[derive(Insertable, Clone, Serialize)]
#[table_name = "refresh_tokens"]
pub struct NewRefreshToken {
    pub jwt_id: uuid::Uuid,
    pub user_id: i32,
    pub invalidated: bool,
    pub used: bool,
    pub expires_at: NaiveDateTime,
}

impl From<NewRefreshToken> for entity::refresh_tokens::ActiveModel {
    fn from(value: NewRefreshToken) -> Self {
        entity::refresh_tokens::ActiveModel {
            id: NotSet,
            created_at: NotSet,
            jwt_id: Set(value.jwt_id),
            user_id: Set(value.user_id),
            invalidated: Set(value.invalidated),
            used: Set(value.used),
            expires_at: Set(value.expires_at),
        }
    }
}

#[derive(Insertable, Clone, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub display_name: String,
    pub password: String,
    pub role: UserRole,
}

impl From<NewUser> for entity::users::ActiveModel {
    fn from(value: NewUser) -> Self {
        entity::users::ActiveModel {
            email: Set(value.email),
            display_name: Set(value.display_name),
            password: Set(value.password),
            role: Set(value.role.into()),
            updated_at: NotSet,
            id: NotSet,
            created_at: NotSet,
        }
    }
}

#[derive(Insertable, Clone, Serialize)]
#[table_name = "home_page_links"]
pub struct NewHomePageLink {
    pub name: String,
    pub target: String,
    pub image: String,
}

#[derive(Insertable, Clone, Serialize)]
#[table_name = "categories"]
pub struct NewCategory {
    pub name: String,
}

#[derive(Insertable, Clone, Serialize)]
#[table_name = "blog_posts_categories"]
pub struct NewBlogPostCategory {
    pub category_id: i32,
    pub blog_post_id: i32,
}

#[derive(Insertable, Clone, Serialize)]
#[table_name = "technologies"]
pub struct NewTechnology {
    pub name: String,
}

#[derive(Insertable, Clone, Serialize)]
#[table_name = "projects_technologies"]
pub struct NewProjectTechnology {
    pub technology_id: i32,
    pub project_id: i32,
}

#[derive(Insertable, Clone, Serialize)]
#[table_name = "uploaded_images"]
pub struct NewUploadedImage {
    pub extension: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub used_where: Option<String>,
    pub user_id: i32,
    pub path: String,
    pub url: String,
}

#[derive(Insertable, Clone, Serialize)]
#[table_name = "text_bodies"]
pub struct NewTextBody {
    pub title: Option<String>,
    pub slug: String,
    pub body: String,
    pub url_used: Option<String>,
}

#[derive(Insertable, Clone, Serialize)]
#[table_name = "identification_cookies"]
pub struct NewIdentificationCookie {
    pub token: String,
    pub id_hash: String,
    pub expires_at: NaiveDateTime,
}

#[derive(Insertable, Clone, Serialize)]
#[table_name = "verify_email_tokens"]
pub struct NewVerifyEmailToken {
    pub token: String,
    pub user_id: i32,
    pub email: String,
    pub old_email: Option<String>,
    pub expires_at: NaiveDateTime,
}

impl From<NewVerifyEmailToken> for entity::verify_email_tokens::ActiveModel {
    fn from(value: NewVerifyEmailToken) -> Self {
        entity::verify_email_tokens::ActiveModel {
            id: NotSet,
            created_at: NotSet,
            user_id: Set(value.user_id),
            invalidated: NotSet,
            used: NotSet,
            expires_at: Set(value.expires_at),
            token: Set(value.token),
            email: Set(value.email),
            old_email: Set(value.old_email),
        }
    }
}

#[derive(Insertable, Clone, Serialize)]
#[table_name = "page_views"]
pub struct NewPageView {
    pub page_url: String,
    pub user_agent: Option<String>,
    pub id_hash: String,
    pub registered: bool,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub country_code: Option<String>,
}

#[derive(Insertable, Clone, Serialize)]
#[table_name = "blog_post_comment_ratings"]
pub struct NewBlogPostCommentRating {
    pub is_like: bool,
    pub user_id: i32,
    pub blog_post_comment_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Clone, Serialize)]
#[table_name = "blog_post_comment_flags"]
pub struct NewBlogPostCommentFlag {
    pub reason: String,
    pub user_id: i32,
    pub blog_post_comment_id: i32,
}

#[derive(Insertable, Clone, Serialize)]
#[table_name = "admin_logs"]
pub struct NewAdminLog {
    pub object_id: String,
    pub user_id: i32,
    pub label: String,
    pub model: String,
    pub action: AdminLogAction,
    pub new_data: Option<String>,
    pub old_data: Option<String>,
    pub base_link: String,
}

#[derive(Insertable, Clone, Serialize)]
#[table_name = "change_password_tokens"]
pub struct NewChangePasswordToken {
    pub token: String,
    pub user_id: i32,
    pub expires_at: NaiveDateTime,
}
impl From<NewChangePasswordToken> for entity::change_password_tokens::ActiveModel {
    fn from(value: NewChangePasswordToken) -> Self {
        entity::change_password_tokens::ActiveModel {
            id: NotSet,
            user_id: Set(value.user_id),
            expires_at: Set(value.expires_at),
            token: Set(value.token),
            invalidated: NotSet,
            used: NotSet,
            created_at: NotSet,
        }
    }
}
