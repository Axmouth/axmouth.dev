use serde::{Deserialize, Serialize};

use crate::options::{
    AdminLogSortType, BlogPostCommentFlagSortType, BlogPostCommentRatingSortType,
    BlogPostCommentSortType, BlogPostSortType, CategorySortType, ChangePasswordTokenSortType,
    HomePageLinkSortType, IdentificationCookieSortType, PageViewSortType, ProjectSortType,
    RefreshTokenSortType, TechnologySortType, TextBodySortType, UploadedImageSortType,
    UserSortType, VerifyEmailTokenSortType,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllBlogPostCommentsQuery {
    pub post: Option<i32>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub author: Option<i32>,
    pub sort_type: Option<BlogPostCommentSortType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllBlogPostsQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub author: Option<i32>,
    pub category_id: Option<i32>,
    pub category_name: Option<String>,
    pub published: Option<bool>,
    pub sort_type: Option<BlogPostSortType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllProjectsQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub technology_id: Option<i32>,
    pub technology_name: Option<String>,
    pub published: Option<bool>,
    pub sort_type: Option<ProjectSortType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllUsersQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<UserSortType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllRefreshTokensQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<RefreshTokenSortType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllTextBodiesQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<TextBodySortType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllUploadedImagesQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<UploadedImageSortType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllIdentificationCookiesQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<IdentificationCookieSortType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllVerifyEmailTokensQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<VerifyEmailTokenSortType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllPageViewsQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<PageViewSortType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetPageViewsQuery {
    pub distinct: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllBlogPostCommentRatingsQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<BlogPostCommentRatingSortType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllBlogPostCommentFlagsQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<BlogPostCommentFlagSortType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllAdminLogsQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<AdminLogSortType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllHomePageLinksQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<HomePageLinkSortType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllTechnologiesQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<TechnologySortType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllCategoriesQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<CategorySortType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllChangePasswordTokensQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<ChangePasswordTokenSortType>,
}
