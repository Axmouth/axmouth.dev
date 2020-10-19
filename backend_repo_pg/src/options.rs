use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SortOrder {
    ASC,
    DESC,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BlogPostCommentSortType {
    CreatedAt,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlogPostCommentSort {
    pub sort_type: Option<BlogPostCommentSortType>,
    pub order: Option<SortOrder>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BlogPostSortType {
    CreatedAt,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlogPostSort {
    pub sort_type: Option<BlogPostSortType>,
    pub order: Option<SortOrder>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum HomePageLinkSortType {
    CreatedAt,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HomePageLinkSort {
    pub sort_type: Option<HomePageLinkSortType>,
    pub order: Option<SortOrder>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ProjectSortType {
    CreatedAt,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectSort {
    pub sort_type: Option<ProjectSortType>,
    pub order: Option<SortOrder>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum UserSortType {
    CreatedAt,
    DisplayName,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserSort {
    pub sort_type: Option<UserSortType>,
    pub order: Option<SortOrder>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum RefreshTokenSortType {
    CreatedAt,
    ExpiresAt,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RefreshTokenSort {
    pub sort_type: Option<RefreshTokenSortType>,
    pub order: Option<SortOrder>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TextBodySortType {
    CreatedAt,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TextBodySort {
    pub sort_type: Option<TextBodySortType>,
    pub order: Option<SortOrder>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum UploadedImageSortType {
    CreatedAt,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UploadedImageSort {
    pub sort_type: Option<UploadedImageSortType>,
    pub order: Option<SortOrder>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum IdentificationCookieSortType {
    CreatedAt,
    ExpiresAt,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IdentificationCookieSort {
    pub sort_type: Option<IdentificationCookieSortType>,
    pub order: Option<SortOrder>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum VerifyEmailTokenSortType {
    CreatedAt,
    ExpiresAt,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VerifyEmailTokenSort {
    pub sort_type: Option<VerifyEmailTokenSortType>,
    pub order: Option<SortOrder>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PageViewSortType {
    CreatedAt,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PageViewSort {
    pub sort_type: Option<PageViewSortType>,
    pub order: Option<SortOrder>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BlogPostCommentRatingSortType {
    CreatedAt,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlogPostCommentRatingSort {
    pub sort_type: Option<BlogPostCommentRatingSortType>,
    pub order: Option<SortOrder>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BlogPostCommentFlagSortType {
    CreatedAt,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlogPostCommentFlagSort {
    pub sort_type: Option<BlogPostCommentFlagSortType>,
    pub order: Option<SortOrder>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum AdminLogSortType {
    CreatedAt,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AdminLogSort {
    pub sort_type: Option<AdminLogSortType>,
    pub order: Option<SortOrder>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TechnologySortType {
    CreatedAt,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TechnologySort {
    pub sort_type: Option<TechnologySortType>,
    pub order: Option<SortOrder>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum CategorySortType {
    CreatedAt,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CategorySort {
    pub sort_type: Option<CategorySortType>,
    pub order: Option<SortOrder>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ChangePasswordTokenSortType {
    CreatedAt,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChangePasswordTokenSort {
    pub sort_type: Option<ChangePasswordTokenSortType>,
    pub order: Option<SortOrder>,
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
