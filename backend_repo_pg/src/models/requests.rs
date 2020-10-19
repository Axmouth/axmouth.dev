use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBlogPostCommentRequest {
    #[validate(length(min = 1, max = 2500))]
    pub body: String,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBlogPostRequest {
    #[validate(length(min = 1, max = 5500))]
    pub body: String,
    pub categories: Option<Vec<String>>,
    pub published: Option<bool>,
    pub description: Option<Option<String>>,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectRequest {
    #[validate(length(min = 1, max = 5500))]
    pub body: Option<String>,
    pub technologies: Option<Vec<String>>,
    pub description: Option<Option<String>>,
    pub cover_image: Option<Option<String>>,
    pub name: Option<String>,
    pub published: Option<bool>,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateBlogPostCommentRequest {
    pub post_id: i32,
    #[validate(length(min = 1, max = 2500))]
    pub body: String,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateBlogPostRequest {
    #[validate(length(min = 1, max = 150))]
    pub title: String,
    #[validate(length(min = 1, max = 5500))]
    pub body: String,
    pub categories: Vec<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRequest {
    #[validate(length(min = 1, max = 5500))]
    pub body: String,
    pub technologies: Vec<String>,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub name: String,
}
#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 35))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRequest {
    #[validate(length(min = 6, max = 999))]
    pub token: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmailConfirmRequest {
    #[validate(length(min = 6, max = 999))]
    pub token: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmailConfirmEmailRequest {
    #[validate(length(min = 6, max = 999))]
    pub token: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 25))]
    pub display_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 35))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PasswordChangeEmailRequest {
    #[validate(length(min = 6, max = 999))]
    pub token: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PasswordChangeRequest {
    #[validate(length(min = 6, max = 999))]
    pub token: String,
    #[validate(length(min = 6, max = 35))]
    pub old_password: String,
    #[validate(length(min = 6, max = 35))]
    pub new_password: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateHomePageLinkRequest {
    #[validate(length(min = 1, max = 55))]
    pub name: String,
    #[validate(length(min = 6, max = 1550))]
    pub image: String,
    #[validate(length(min = 6, max = 1550))]
    pub target: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHomePageLinkRequest {
    #[validate(length(min = 1, max = 55))]
    pub name: Option<String>,
    #[validate(length(min = 6, max = 1550))]
    pub image: Option<String>,
    #[validate(length(min = 6, max = 1550))]
    pub target: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateTechnologyRequest {
    #[validate(length(min = 1, max = 55))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTechnologyRequest {
    #[validate(length(min = 1, max = 55))]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateCategoryRequest {
    #[validate(length(min = 1, max = 55))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCategoryRequest {
    #[validate(length(min = 1, max = 55))]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateTextBodyRequest {
    pub title: Option<String>,
    pub slug: String,
    pub body: String,
    pub url_used: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTextBodyRequest {
    pub title: Option<Option<String>>,
    pub slug: Option<String>,
    pub body: Option<String>,
    pub url_used: Option<Option<String>>,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SendContactEmailRequest {
    #[validate(length(min = 1, max = 78))]
    pub subject: String,
    #[validate(email)]
    pub from_email: String,
    #[validate(length(min = 1, max = 5000))]
    pub body: String,
    #[validate(length(min = 1, max = 5000))]
    pub captcha_token: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestVerificationEmailRequest {
    #[validate(email)]
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VerifyEmailRequest {
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 1, max = 5000))]
    pub token: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestResetPasswordEmailRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordRequest {
    #[validate(length(min = 1, max = 5000))]
    pub token: String,
    #[validate(length(min = 1, max = 5000))]
    pub new_password: String,
}
