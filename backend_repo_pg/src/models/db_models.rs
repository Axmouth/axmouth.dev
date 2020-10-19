use crate::extra::UserRole;
use crate::schema::{
    admin_logs, blog_post_comment_flags, blog_post_comment_ratings, blog_post_comments, blog_posts,
    blog_posts_categories, categories, change_password_tokens, home_page_links,
    identification_cookies, page_views, projects, projects_technologies, refresh_tokens,
    technologies, text_bodies, uploaded_images, users, verify_email_tokens,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable, Associations)]
#[belongs_to(parent = "User", foreign_key = "author_id")]
#[belongs_to(parent = "BlogPost", foreign_key = "post_id")]
#[table_name = "blog_post_comments"]
pub struct BlogPostComment {
    pub id: i32,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub author_id: i32,
    pub post_id: i32,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable, Associations)]
#[belongs_to(parent = "User", foreign_key = "author_id")]
#[table_name = "blog_posts"]
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub published: bool,
    pub author_id: i32,
    pub description: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable)]
#[table_name = "projects"]
pub struct Project {
    pub id: i32,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub name: String,
    pub published: bool,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable, Associations)]
#[belongs_to(parent = "User", foreign_key = "user_id")]
#[table_name = "refresh_tokens"]
pub struct RefreshToken {
    pub id: uuid::Uuid,
    pub jwt_id: uuid::Uuid,
    pub user_id: i32,
    pub invalidated: bool,
    pub used: bool,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable, Associations)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub role: UserRole,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable, Associations)]
#[table_name = "home_page_links"]
pub struct HomePageLink {
    pub id: i32,
    pub name: String,
    pub target: String,
    pub image: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable, Associations)]
#[table_name = "categories"]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable, Associations)]
#[belongs_to(parent = "Category", foreign_key = "category_id")]
#[belongs_to(parent = "BlogPost", foreign_key = "blog_post_id")]
#[table_name = "blog_posts_categories"]
pub struct BlogPostCategory {
    pub id: i32,
    pub category_id: i32,
    pub blog_post_id: i32,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable, Associations)]
#[table_name = "technologies"]
pub struct Technology {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable, Associations)]
#[belongs_to(parent = "Technology", foreign_key = "technology_id")]
#[belongs_to(parent = "Project", foreign_key = "project_id")]
#[table_name = "projects_technologies"]
pub struct ProjectTechnology {
    pub id: i32,
    pub technology_id: i32,
    pub project_id: i32,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable, Associations)]
#[belongs_to(parent = "User", foreign_key = "user_id")]
#[table_name = "uploaded_images"]
pub struct UploadedImage {
    pub id: i32,
    pub extension: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub used_where: Option<String>,
    pub created_at: NaiveDateTime,
    pub user_id: i32,
    pub path: String,
    pub url: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable, Associations)]
#[table_name = "text_bodies"]
pub struct TextBody {
    pub id: i32,
    pub title: Option<String>,
    pub slug: String,
    pub body: String,
    pub url_used: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable, Associations)]
#[table_name = "identification_cookies"]
pub struct IdentificationCookie {
    pub id: i32,
    pub token: String,
    pub id_hash: String,
    pub expires_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable, Associations)]
#[table_name = "verify_email_tokens"]
#[belongs_to(parent = "User", foreign_key = "user_id")]
pub struct VerifyEmailToken {
    pub id: i32,
    pub token: String,
    pub user_id: i32,
    pub email: String,
    pub old_email: Option<String>,
    pub invalidated: bool,
    pub used: bool,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable, Associations)]
#[table_name = "page_views"]
pub struct PageView {
    pub id: i32,
    pub page_url: String,
    pub user_agent: String,
    pub user_location: String,
    pub id_hash: String,
    pub registered: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable, Associations)]
#[belongs_to(parent = "BlogPostComment", foreign_key = "blog_post_comment_id")]
#[belongs_to(parent = "User", foreign_key = "user_id")]
#[table_name = "blog_post_comment_ratings"]
pub struct BlogPostCommentRating {
    pub id: i32,
    pub is_like: bool,
    pub user_id: i32,
    pub blog_post_comment_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable, Associations)]
#[belongs_to(parent = "BlogPostComment", foreign_key = "blog_post_comment_id")]
#[belongs_to(parent = "User", foreign_key = "user_id")]
#[table_name = "blog_post_comment_flags"]
pub struct BlogPostCommentFlag {
    pub id: i32,
    pub reason: String,
    pub user_id: i32,
    pub blog_post_comment_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable, Associations)]
#[belongs_to(parent = "User", foreign_key = "user_id")]
#[table_name = "admin_logs"]
pub struct AdminLog {
    pub id: i32,
    pub change_message: String,
    pub object_id: String,
    pub user_id: i32,
    pub label: String,
    pub model: String,
    pub action_flag: i32,
    pub action_time: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq, Identifiable, Associations)]
#[table_name = "change_password_tokens"]
#[belongs_to(parent = "User", foreign_key = "user_id")]
pub struct ChangePasswordToken {
    pub id: i32,
    pub token: String,
    pub user_id: i32,
    pub invalidated: bool,
    pub used: bool,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

//TODO Implement Domain/Insertable/Changeset models bellow
//TODO Finalize and add to DB bellow
