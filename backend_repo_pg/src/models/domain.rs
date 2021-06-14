use crate::extra::{AdminLogAction, SearchItemType, UserRole};
use crate::models::db_models;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BlogPostComment {
    pub id: i32,
    pub body: String,
    pub author: User,
    pub post_id: i32,
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<NaiveDateTime>,
}

impl BlogPostComment {
    pub fn from(comment: db_models::BlogPostComment, author: db_models::User) -> Self {
        let mut author = User::from(author);
        author.email = None;
        Self {
            author,
            body: comment.body,
            created_at: comment.created_at,
            id: comment.id,
            updated_at: comment.updated_at,
            post_id: comment.post_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<NaiveDateTime>,
    pub published: bool,
    pub author: User,
    pub categories: Vec<String>,
    pub slug: String,
}

impl BlogPost {
    pub fn from(
        post: db_models::BlogPost,
        author: db_models::User,
        categories_list: Vec<String>,
    ) -> Self {
        let mut author = User::from(author);
        author.email = None;
        Self {
            author,
            title: post.title,
            body: post.body,
            created_at: post.created_at,
            id: post.id,
            published: post.published,
            updated_at: post.updated_at,
            categories: categories_list,
            slug: post.slug,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: i32,
    pub body: String,
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<NaiveDateTime>,
    pub technologies: Vec<String>,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub name: String,
    pub published: bool,
    pub slug: String,
}

impl Project {
    pub fn from(project: db_models::Project, technologies: Vec<String>) -> Self {
        Self {
            body: project.body,
            created_at: project.created_at,
            id: project.id,
            updated_at: project.updated_at,
            technologies,
            description: project.description,
            cover_image: project.cover_image,
            name: project.name,
            published: project.published,
            slug: project.slug,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub role: UserRole,
    pub created_at: NaiveDateTime,
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub fn from(user: db_models::User) -> Self {
        Self {
            created_at: user.created_at,
            display_name: user.display_name,
            id: user.id,
            role: user.role,
            email: Some(user.email),
            updated_at: user.updated_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HomePageLink {
    pub id: i32,
    pub name: String,
    pub target: String,
    pub image: String,
}

impl HomePageLink {
    pub fn from(link: db_models::HomePageLink) -> Self {
        Self {
            id: link.id,
            name: link.name,
            target: link.target,
            image: link.image,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Technology {
    pub id: i32,
    pub name: String,
}

impl Technology {
    pub fn from(tech: db_models::Technology) -> Self {
        Self {
            id: tech.id,
            name: tech.name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i32,
    pub name: String,
}

impl Category {
    pub fn from(category: db_models::Category) -> Self {
        Self {
            id: category.id,
            name: category.name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RefreshToken {
    pub id: uuid::Uuid,
    pub jwt_id: uuid::Uuid,
    pub user_id: i32,
    pub invalidated: bool,
    pub used: bool,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

impl RefreshToken {
    pub fn from(token: db_models::RefreshToken) -> Self {
        Self {
            created_at: token.created_at,
            expires_at: token.expires_at,
            id: token.id,
            invalidated: token.invalidated,
            jwt_id: token.jwt_id,
            used: token.used,
            user_id: token.user_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UploadedImage {
    pub id: i32,
    pub extension: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_where: Option<String>,
    pub created_at: NaiveDateTime,
    pub user_id: i32,
}

impl UploadedImage {
    pub fn from(image: db_models::UploadedImage) -> Self {
        Self {
            created_at: image.created_at,
            extension: image.extension,
            height: image.height,
            id: image.id,
            width: image.width,
            used_where: image.used_where,
            user_id: image.user_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextBody {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub slug: String,
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_used: Option<String>,
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<NaiveDateTime>,
}

impl TextBody {
    pub fn from(text: db_models::TextBody) -> Self {
        Self {
            id: text.id,
            title: text.title,
            slug: text.slug,
            body: text.body,
            url_used: text.url_used,
            created_at: text.created_at,
            updated_at: text.updated_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IdentificationCookie {
    pub id: i32,
    pub token: String,
    pub id_hash: String,
    pub expires_at: NaiveDateTime,
}

impl IdentificationCookie {
    pub fn from(cookie: db_models::IdentificationCookie) -> Self {
        Self {
            id: cookie.id,
            token: cookie.token,
            id_hash: cookie.id_hash,
            expires_at: cookie.expires_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VerifyEmailToken {
    pub id: i32,
    pub token: String,
    pub user_id: i32,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_email: Option<String>,
    pub invalidated: bool,
    pub used: bool,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

impl VerifyEmailToken {
    pub fn from(token: db_models::VerifyEmailToken) -> Self {
        Self {
            id: token.id,
            token: token.token,
            user_id: token.user_id,
            email: token.email,
            old_email: token.old_email,
            invalidated: token.invalidated,
            used: token.used,
            created_at: token.created_at,
            expires_at: token.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PageView {
    pub id: i32,
    pub page_url: String,
    pub user_agent: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub country_code: Option<String>,
    pub id_hash: String,
    pub registered: bool,
    pub created_at: NaiveDateTime,
}

impl PageView {
    pub fn from(view: db_models::PageView) -> Self {
        Self {
            id: view.id,
            page_url: view.page_url,
            user_agent: view.user_agent,
            latitude: view.latitude,
            longitude: view.longitude,
            country_code: view.country_code,
            id_hash: view.id_hash,
            registered: view.registered,
            created_at: view.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BlogPostCommentRating {
    id: i32,
    pub is_like: bool,
    pub user_id: i32,
    pub blog_post_comment_id: i32,
    pub created_at: NaiveDateTime,
}

impl BlogPostCommentRating {
    pub fn from(rating: db_models::BlogPostCommentRating) -> Self {
        Self {
            id: rating.id,
            user_id: rating.user_id,
            is_like: rating.is_like,
            blog_post_comment_id: rating.blog_post_comment_id,
            created_at: rating.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BlogPostCommentFlag {
    pub id: i32,
    pub reason: String,
    pub user_id: i32,
    pub blog_post_comment_id: i32,
    pub created_at: NaiveDateTime,
}

impl BlogPostCommentFlag {
    pub fn from(flag: db_models::BlogPostCommentFlag) -> Self {
        Self {
            id: flag.id,
            user_id: flag.user_id,
            reason: flag.reason,
            blog_post_comment_id: flag.blog_post_comment_id,
            created_at: flag.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AdminLog {
    pub id: i32,
    pub object_id: String,
    pub user: User,
    pub label: String,
    pub model: String,
    pub action_time: NaiveDateTime,
    pub action: AdminLogAction,
    pub new_data: Option<String>,
    pub old_data: Option<String>,
    pub base_link: String,
}

impl AdminLog {
    pub fn from(log: db_models::AdminLog, user: db_models::User) -> Self {
        let mut user = User::from(user);
        user.email = None;
        Self {
            id: log.id,
            object_id: log.object_id,
            user,
            label: log.label,
            model: log.model,
            action_time: log.action_time,
            action: log.action,
            new_data: log.new_data,
            old_data: log.old_data,
            base_link: log.base_link,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordToken {
    pub id: i32,
    pub token: String,
    pub user_id: i32,
    pub invalidated: bool,
    pub used: bool,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

impl ChangePasswordToken {
    pub fn from(change_pass_token: db_models::ChangePasswordToken) -> Self {
        Self {
            id: change_pass_token.id,
            token: change_pass_token.token,
            user_id: change_pass_token.user_id,
            invalidated: change_pass_token.invalidated,
            used: change_pass_token.used,
            created_at: change_pass_token.created_at,
            expires_at: change_pass_token.expires_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SearchItem {
    pub title: String,
    pub description: String,
    pub item_type: SearchItemType,
    pub link: String,
}

impl SearchItem {
    pub fn from(search_item: db_models::SearchItem) -> Self {
        Self {
            title: search_item.title,
            description: search_item.description,
            item_type: search_item.item_type,
            link: search_item.link,
        }
    }
}
