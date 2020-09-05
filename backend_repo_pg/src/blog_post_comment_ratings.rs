use crate::errors::PgRepoError;
use crate::filters::GetAllBlogPostCommentRatingsFilter;
use crate::models::{db_models, domain};
use crate::options::{BlogPostCommentRatingSort, PaginationOptions};
use crate::schema::blog_post_comment_ratings;
use crate::{change_sets::UpdateBlogPostCommentRating, insertables::NewBlogPostCommentRating};
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

#[derive(Clone)]
pub struct BlogPostCommentRatingRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl BlogPostCommentRatingRepo {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn insert_one(
        &self,
        new_blog_post_comment_rating: NewBlogPostCommentRating,
    ) -> Result<domain::BlogPostCommentRating, PgRepoError> {
        let conn = self.pool.get()?;
        let query = diesel::insert_into(blog_post_comment_ratings::table)
            .values(&new_blog_post_comment_rating);
        let result = tokio::task::block_in_place(move || query.get_result(&conn))?;
        Ok(domain::BlogPostCommentRating::from(result))
    }

    pub async fn update_one(
        &self,
        id_value: i32,
        updated_blog_post_comment_rating: UpdateBlogPostCommentRating,
    ) -> Result<domain::BlogPostCommentRating, PgRepoError> {
        use crate::schema::blog_post_comment_ratings::dsl::{blog_post_comment_ratings, id};
        let conn = self.pool.get()?;
        let query = diesel::update(blog_post_comment_ratings.filter(id.eq(id_value)))
            .set(&updated_blog_post_comment_rating);
        let result = tokio::task::block_in_place(move || query.get_result(&conn))?;
        Ok(domain::BlogPostCommentRating::from(result))
    }

    pub async fn delete_one(&self, id_value: i32) -> Result<usize, PgRepoError> {
        use crate::schema::blog_post_comment_ratings::dsl::{blog_post_comment_ratings, id};
        let conn = self.pool.get()?;
        let query = diesel::delete(blog_post_comment_ratings.filter(id.eq(id_value)));
        Ok(query.execute(&conn)?)
    }

    pub async fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::BlogPostCommentRating>, PgRepoError> {
        use crate::schema::blog_post_comment_ratings::dsl::{blog_post_comment_ratings, id};

        let conn = self.pool.get()?;
        let query = blog_post_comment_ratings
            .filter(id.eq(id_value))
            .select(blog_post_comment_ratings::all_columns());
        let blog_post_comment_rating: db_models::BlogPostCommentRating =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::BlogPostCommentRating::from(
            blog_post_comment_rating,
        )))
    }

    pub async fn find(
        &self,
        filter: GetAllBlogPostCommentRatingsFilter,
        sort: BlogPostCommentRatingSort,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::BlogPostCommentRating>, PgRepoError> {
        use crate::schema::blog_post_comment_ratings::dsl::{blog_post_comment_ratings, id};
        let q = blog_post_comment_ratings
            .select(blog_post_comment_ratings::all_columns())
            .into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = self.pool.get()?;
        let results: Vec<db_models::BlogPostCommentRating> =
            tokio::task::block_in_place(move || q.load(&conn))?;

        Ok(results
            .into_iter()
            .map(|blog_post_comment_rating| {
                domain::BlogPostCommentRating::from(blog_post_comment_rating)
            })
            .collect::<Vec<_>>())
    }
}
