use axum_derive::ValidatedExtractedQuery;
use serde::{Deserialize, Serialize};

use crate::{
    extra::{AdminLogAction, SearchItemType},
    options::{
        AdminLogSortType, BlogPostCommentFlagSortType, BlogPostCommentRatingSortType,
        BlogPostCommentSortType, BlogPostSortType, CategorySortType, ChangePasswordTokenSortType,
        HomePageLinkSortType, IdentificationCookieSortType, PageViewSortType, PaginationOptions,
        ProjectSortType, RefreshTokenSortType, TechnologySortType, TextBodySortType,
        UploadedImageSortType, UserSortType, VerifyEmailTokenSortType,
    },
};

pub trait PaginatedQuery {
    fn pagination_options(&self) -> PaginationOptions;
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetAllBlogPostCommentsQuery {
    pub post: Option<i32>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub author: Option<i32>,
    pub sort_type: Option<BlogPostCommentSortType>,
}

impl PaginatedQuery for GetAllBlogPostCommentsQuery {
    fn pagination_options(&self) -> PaginationOptions {
        PaginationOptions {
            page: self.page,
            page_size: self.page_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
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

impl PaginatedQuery for GetAllBlogPostsQuery {
    fn pagination_options(&self) -> PaginationOptions {
        PaginationOptions {
            page: self.page,
            page_size: self.page_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetBlogPostQuery {
    pub use_slug: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetAllProjectsQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub technology_id: Option<i32>,
    pub technology_name: Option<String>,
    pub published: Option<bool>,
    pub sort_type: Option<ProjectSortType>,
}

impl PaginatedQuery for GetAllProjectsQuery {
    fn pagination_options(&self) -> PaginationOptions {
        PaginationOptions {
            page: self.page,
            page_size: self.page_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetProjectQuery {
    pub use_slug: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetAllUsersQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<UserSortType>,
}

impl PaginatedQuery for GetAllUsersQuery {
    fn pagination_options(&self) -> PaginationOptions {
        PaginationOptions {
            page: self.page,
            page_size: self.page_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetAllRefreshTokensQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<RefreshTokenSortType>,
}

impl PaginatedQuery for GetAllRefreshTokensQuery {
    fn pagination_options(&self) -> PaginationOptions {
        PaginationOptions {
            page: self.page,
            page_size: self.page_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetAllTextBodiesQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<TextBodySortType>,
}

impl PaginatedQuery for GetAllTextBodiesQuery {
    fn pagination_options(&self) -> PaginationOptions {
        PaginationOptions {
            page: self.page,
            page_size: self.page_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetAllUploadedImagesQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<UploadedImageSortType>,
}

impl PaginatedQuery for GetAllUploadedImagesQuery {
    fn pagination_options(&self) -> PaginationOptions {
        PaginationOptions {
            page: self.page,
            page_size: self.page_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetAllIdentificationCookiesQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<IdentificationCookieSortType>,
}

impl PaginatedQuery for GetAllIdentificationCookiesQuery {
    fn pagination_options(&self) -> PaginationOptions {
        PaginationOptions {
            page: self.page,
            page_size: self.page_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetAllVerifyEmailTokensQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<VerifyEmailTokenSortType>,
}

impl PaginatedQuery for GetAllVerifyEmailTokensQuery {
    fn pagination_options(&self) -> PaginationOptions {
        PaginationOptions {
            page: self.page,
            page_size: self.page_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetAllPageViewsQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<PageViewSortType>,
}

impl PaginatedQuery for GetAllPageViewsQuery {
    fn pagination_options(&self) -> PaginationOptions {
        PaginationOptions {
            page: self.page,
            page_size: self.page_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetPageViewsQuery {
    pub distinct: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetAllBlogPostCommentRatingsQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<BlogPostCommentRatingSortType>,
}

impl PaginatedQuery for GetAllBlogPostCommentRatingsQuery {
    fn pagination_options(&self) -> PaginationOptions {
        PaginationOptions {
            page: self.page,
            page_size: self.page_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetAllBlogPostCommentFlagsQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<BlogPostCommentFlagSortType>,
}

impl PaginatedQuery for GetAllBlogPostCommentFlagsQuery {
    fn pagination_options(&self) -> PaginationOptions {
        PaginationOptions {
            page: self.page,
            page_size: self.page_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetAllAdminLogsQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<AdminLogSortType>,
    pub action: Option<AdminLogAction>,
}

impl PaginatedQuery for GetAllAdminLogsQuery {
    fn pagination_options(&self) -> PaginationOptions {
        PaginationOptions {
            page: self.page,
            page_size: self.page_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetAllHomePageLinksQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<HomePageLinkSortType>,
}

impl PaginatedQuery for GetAllHomePageLinksQuery {
    fn pagination_options(&self) -> PaginationOptions {
        PaginationOptions {
            page: self.page,
            page_size: self.page_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetAllTechnologiesQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<TechnologySortType>,
}

impl PaginatedQuery for GetAllTechnologiesQuery {
    fn pagination_options(&self) -> PaginationOptions {
        PaginationOptions {
            page: self.page,
            page_size: self.page_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetAllCategoriesQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<CategorySortType>,
}

impl PaginatedQuery for GetAllCategoriesQuery {
    fn pagination_options(&self) -> PaginationOptions {
        PaginationOptions {
            page: self.page,
            page_size: self.page_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetAllChangePasswordTokensQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_type: Option<ChangePasswordTokenSortType>,
}

impl PaginatedQuery for GetAllChangePasswordTokensQuery {
    fn pagination_options(&self) -> PaginationOptions {
        PaginationOptions {
            page: self.page,
            page_size: self.page_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, ValidatedExtractedQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetAllSearchItemsQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub search_text: Option<String>,
    pub r#type: Option<SearchItemType>,
}

impl PaginatedQuery for GetAllSearchItemsQuery {
    fn pagination_options(&self) -> PaginationOptions {
        PaginationOptions {
            page: self.page,
            page_size: self.page_size,
        }
    }
}
