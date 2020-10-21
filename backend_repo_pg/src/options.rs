use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BlogPostCommentSortType {
    CreatedAtAsc,
    CreatedAtDesc,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BlogPostSortType {
    CreatedAtAsc,
    CreatedAtDesc,
    TitleAsc,
    TitleDesc,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum HomePageLinkSortType {
    CreatedAtAsc,
    CreatedAtDesc,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ProjectSortType {
    CreatedAtAsc,
    CreatedAtDesc,
    NameAsc,
    NameDesc,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum UserSortType {
    CreatedAtDesc,
    CreatedAtAsc,
    DisplayNameDesc,
    DisplayNameAsc,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum RefreshTokenSortType {
    CreatedAtDesc,
    CreatedAtAsc,
    ExpiresDesc,
    ExpiresAsc,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TextBodySortType {
    CreatedAtDesc,
    CreatedAtAsc,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum UploadedImageSortType {
    CreatedAtDesc,
    CreatedAtAsc,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum IdentificationCookieSortType {
    CreatedAtDesc,
    CreatedAtAsc,
    ExpiresAtDesc,
    ExpiresAtAsc,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum VerifyEmailTokenSortType {
    CreatedAtAsc,
    CreatedAtDesc,
    ExpiresAtAsc,
    ExpiresAtDesc,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PageViewSortType {
    CreatedAtAsc,
    CreatedAtDesc,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BlogPostCommentRatingSortType {
    CreatedAtAsc,
    CreatedAtDesc,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BlogPostCommentFlagSortType {
    CreatedAtAsc,
    CreatedAtDesc,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum AdminLogSortType {
    CreatedAtAsc,
    CreatedAtDesc,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TechnologySortType {
    CreatedAtAsc,
    CreatedAtDesc,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum CategorySortType {
    CreatedAtAsc,
    CreatedAtDesc,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ChangePasswordTokenSortType {
    CreatedAtAsc,
    CreatedAtDesc,
}

pub struct PaginationOptions {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

impl PaginationOptions {
    pub fn new(page_size: Option<i64>, page: Option<i64>) -> Self {
        Self { page, page_size }
    }
}
