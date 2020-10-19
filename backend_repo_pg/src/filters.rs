use crate::models::queries::{
    GetAllAdminLogsQuery, GetAllBlogPostCommentFlagsQuery, GetAllBlogPostCommentRatingsQuery,
    GetAllBlogPostCommentsQuery, GetAllBlogPostsQuery, GetAllCategoriesQuery,
    GetAllChangePasswordTokensQuery, GetAllHomePageLinksQuery, GetAllIdentificationCookiesQuery,
    GetAllPageViewsQuery, GetAllProjectsQuery, GetAllRefreshTokensQuery, GetAllTechnologiesQuery,
    GetAllTextBodiesQuery, GetAllUploadedImagesQuery, GetAllUsersQuery,
    GetAllVerifyEmailTokensQuery,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAllBlogPostCommentsFilter {
    pub post_id: Option<i32>,
    pub author_id: Option<i32>,
}

impl GetAllBlogPostCommentsFilter {
    pub fn from_query(query: GetAllBlogPostCommentsQuery) -> Self {
        Self {
            author_id: query.author,
            post_id: query.post,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAllBlogPostsFilter {
    pub author_id: Option<i32>,
    pub category_id: Option<i32>,
    pub category_name: Option<String>,
    pub published: Option<bool>,
}

impl GetAllBlogPostsFilter {
    pub fn from_query(query: GetAllBlogPostsQuery) -> Self {
        Self {
            author_id: query.author,
            category_id: query.category_id,
            category_name: query.category_name,
            published: query.published,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAllProjectsFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technology_id: Option<i32>,
    pub technology_name: Option<String>,
    pub published: Option<bool>,
}

impl GetAllProjectsFilter {
    pub fn from_query(query: GetAllProjectsQuery) -> Self {
        Self {
            technology_id: query.technology_id,
            technology_name: query.technology_name,
            published: query.published,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAllRefreshTokens {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technology: Option<i32>,
}

#[derive(Clone, Debug)]
pub struct GetAllUsersFilter {}

impl GetAllUsersFilter {
    pub fn from_query(query: GetAllUsersQuery) -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct GetAllRefreshTokensFilter {}

impl GetAllRefreshTokensFilter {
    pub fn from_query(query: GetAllRefreshTokensQuery) -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct GetAllTextBodiesFilter {}

impl GetAllTextBodiesFilter {
    pub fn from_query(query: GetAllTextBodiesQuery) -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct GetAllUploadedImagesFilter {}

impl GetAllUploadedImagesFilter {
    pub fn from_query(query: GetAllUploadedImagesQuery) -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct GetAllIdentificationCookiesFilter {}

impl GetAllIdentificationCookiesFilter {
    pub fn from_query(query: GetAllIdentificationCookiesQuery) -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct GetAllVerifyEmailTokensFilter {}

impl GetAllVerifyEmailTokensFilter {
    pub fn from_query(query: GetAllVerifyEmailTokensQuery) -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct GetAllPageViewsFilter {}

impl GetAllPageViewsFilter {
    pub fn from_query(query: GetAllPageViewsQuery) -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct GetAllBlogPostCommentRatingsFilter {}

impl GetAllBlogPostCommentRatingsFilter {
    pub fn from_query(query: GetAllBlogPostCommentRatingsQuery) -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct GetAllBlogPostCommentFlagsFilter {}

impl GetAllBlogPostCommentFlagsFilter {
    pub fn from_query(query: GetAllBlogPostCommentFlagsQuery) -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct GetAllAdminLogsFilter {}

impl GetAllAdminLogsFilter {
    pub fn from_query(query: GetAllAdminLogsQuery) -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct GetAllHomePageLinksFilter {}

impl GetAllHomePageLinksFilter {
    pub fn from_query(query: GetAllHomePageLinksQuery) -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct GetAllTechnologiesFilter {}

impl GetAllTechnologiesFilter {
    pub fn from_query(query: GetAllTechnologiesQuery) -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct GetAllCategoriesFilter {}

impl GetAllCategoriesFilter {
    pub fn from_query(query: GetAllCategoriesQuery) -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct GetAllChangePasswordTokensFilter {}

impl GetAllChangePasswordTokensFilter {
    pub fn from_query(query: GetAllChangePasswordTokensQuery) -> Self {
        Self {}
    }
}
