use crate::extra::UserRole;
use crate::schema::*;

use chrono::NaiveDateTime;

#[derive(Insertable)]
#[table_name = "blog_post_comments"]
pub struct NewBlogPostComment {
    pub body: Option<String>,
    pub author_id: i32,
    pub post_id: i32,
}

#[derive(Insertable)]
#[table_name = "blog_posts"]
pub struct NewBlogPost {
    pub title: String,
    pub body: String,
    pub published: bool,
    pub author_id: i32,
    pub description: Option<String>,
}

#[derive(Insertable)]
#[table_name = "projects"]
pub struct NewProject {
    pub body: String,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "refresh_tokens"]
pub struct NewRefreshToken {
    pub jwt_id: uuid::Uuid,
    pub user_id: i32,
    pub invalidated: bool,
    pub used: bool,
    pub expires_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub display_name: String,
    pub password: String,
    pub role: UserRole,
}

#[derive(Insertable)]
#[table_name = "home_page_links"]
pub struct NewHomePageLink {
    pub name: String,
    pub target: String,
    pub image: String,
}

#[derive(Insertable)]
#[table_name = "categories"]
pub struct NewCategory {
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "blog_posts_categories"]
pub struct NewBlogPostCategory {
    pub category_id: i32,
    pub blog_post_id: i32,
}

#[derive(Insertable)]
#[table_name = "technologies"]
pub struct NewTechnology {
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "projects_technologies"]
pub struct NewProjectTechnology {
    pub technology_id: i32,
    pub project_id: i32,
}

#[derive(Insertable)]
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

#[derive(Insertable)]
#[table_name = "text_bodies"]
pub struct NewTextBody {
    pub title: Option<String>,
    pub slug: String,
    pub body: String,
    pub url_used: Option<String>,
}

#[derive(Insertable)]
#[table_name = "identification_cookies"]
pub struct NewIdentificationCookie {
    pub token: String,
    pub id_hash: String,
    pub expires_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "verify_email_tokens"]
pub struct NewVerifyEmailToken {
    pub token: String,
    pub user_id: i32,
    pub email: String,
    pub old_email: Option<String>,
    pub expires_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "page_views"]
pub struct NewPageView {
    pub page_url: String,
    pub user_agent: String,
    pub user_location: String,
    pub id_hash: String,
    pub registered: bool,
}

#[derive(Insertable)]
#[table_name = "blog_post_comment_ratings"]
pub struct NewBlogPostCommentRating {
    pub is_like: bool,
    pub user_id: i32,
    pub blog_post_comment_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "blog_post_comment_flags"]
pub struct NewBlogPostCommentFlag {
    pub reason: String,
    pub user_id: i32,
    pub blog_post_comment_id: i32,
}

#[derive(Insertable)]
#[table_name = "admin_logs"]
pub struct NewAdminLog {
    pub change_message: String,
    pub object_id: String,
    pub user_id: i32,
    pub label: String,
    pub model: String,
    pub action_flag: i32,
}
