use regex::Regex;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref HAS_UPPER_CASE: Regex = Regex::new("[A-Z]").unwrap();
    static ref HAS_LOWER_CASE: Regex = Regex::new("[a-z]").unwrap();
    static ref HAS_DIGIT: Regex = Regex::new("\\d").unwrap();
    static ref HAS_SPECIAL_CHAR: Regex =
        Regex::new("[ !@#$%^&*()_+\\-=\\[\\]{};':\"\\|,.<>/?]").unwrap();
    static ref HAS_NO_SPECIAL_CHAR: Regex =
        Regex::new("^[^!@#$%^&*()_+\\-=\\[\\]{};':\"\\|,.<>/?]+$").unwrap();
    static ref HAS_NO_SPACE_PREFFIX_OR_SUFFIX: Regex = Regex::new("^[^ ]+.*[^ ]+$").unwrap();
}

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
    pub body: Option<String>,
    pub categories: Option<Vec<String>>,
    pub published: Option<bool>,
    pub description: Option<Option<String>>,
    pub slug: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectRequest {
    #[validate(length(min = 1, max = 5500))]
    pub body: Option<String>,
    pub technologies: Option<Vec<String>>,
    pub description: Option<Option<String>>,
    #[validate(url)]
    pub cover_image: Option<Option<String>>,
    pub name: Option<String>,
    pub published: Option<bool>,
    pub slug: Option<String>,
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
    pub slug: String,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRequest {
    #[validate(length(min = 1, max = 5500))]
    pub body: String,
    pub technologies: Vec<String>,
    pub description: Option<String>,
    #[validate(url)]
    pub cover_image: Option<String>,
    pub name: String,
    pub slug: String,
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
    #[validate(regex(
        path = "HAS_NO_SPECIAL_CHAR",
        message = "not allowed to have a special character"
    ))]
    #[validate(regex(
        path = "HAS_NO_SPACE_PREFFIX_OR_SUFFIX",
        message = "not allowed to start or end with whitespace"
    ))]
    #[validate(length(min = 3, max = 25))]
    pub display_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(regex(path = "HAS_UPPER_CASE", message = "requires an upper case character"))]
    #[validate(regex(path = "HAS_LOWER_CASE", message = "requires a lower case character"))]
    #[validate(regex(path = "HAS_DIGIT", message = "requires a numeric character"))]
    #[validate(regex(path = "HAS_SPECIAL_CHAR", message = "requires a special character"))]
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
    #[validate(url)]
    pub image: String,
    #[validate(url)]
    pub target: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHomePageLinkRequest {
    #[validate(length(min = 1, max = 55))]
    pub name: Option<String>,
    #[validate(url)]
    pub image: Option<String>,
    #[validate(url)]
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
pub struct CreatePageViewRequest {
    pub page_url: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub country_code: Option<String>,
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
