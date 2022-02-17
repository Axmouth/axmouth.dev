use crate::errors::PgRepoError;
use crate::filters::GetAllBlogPostCommentRatingsFilter;
use crate::models::{db_models, domain};
use crate::options::{BlogPostCommentRatingSortType, PaginationOptions};
use crate::schema::blog_post_comment_ratings;
use crate::{change_sets::UpdateBlogPostCommentRating, insertables::NewBlogPostCommentRating};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub struct BlogPostCommentRatingRepo<'a> {
    conn: &'a crate::pg_util::RepoConnection,
}

impl<'a> BlogPostCommentRatingRepo<'a> {
    pub fn new(conn: &'a crate::pg_util::RepoConnection) -> Self {
        Self { conn }
    }

    pub fn insert_one(
        &self,
        new_blog_post_comment_rating: NewBlogPostCommentRating,
    ) -> Result<domain::BlogPostCommentRating, diesel::result::Error> {
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(blog_post_comment_ratings::table)
            .values(&new_blog_post_comment_rating);
        let result = query.get_result(conn)?;
        Ok(domain::BlogPostCommentRating::from(result))
    }

    pub async fn update_one(
        &self,
        id_value: i32,
        updated_blog_post_comment_rating: UpdateBlogPostCommentRating,
    ) -> Result<domain::BlogPostCommentRating, diesel::result::Error> {
        use crate::schema::blog_post_comment_ratings::dsl::{blog_post_comment_ratings, id};
        let conn = &self.conn.pg_conn;
        let query = diesel::update(blog_post_comment_ratings.filter(id.eq(id_value)))
            .set(&updated_blog_post_comment_rating);
        let result = query.get_result(conn)?;
        Ok(domain::BlogPostCommentRating::from(result))
    }

    pub fn delete_one(&self, id_value: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::blog_post_comment_ratings::dsl::{blog_post_comment_ratings, id};
        let conn = &self.conn.pg_conn;
        let query = diesel::delete(blog_post_comment_ratings.filter(id.eq(id_value)));
        Ok(query.execute(conn)?)
    }

    pub fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::BlogPostCommentRating>, diesel::result::Error> {
        use crate::schema::blog_post_comment_ratings::dsl::{blog_post_comment_ratings, id};

        let conn = &self.conn.pg_conn;
        let query = blog_post_comment_ratings
            .filter(id.eq(id_value))
            .select(blog_post_comment_ratings::all_columns());
        let blog_post_comment_rating: db_models::BlogPostCommentRating =
            match query.first(conn).optional()? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::BlogPostCommentRating::from(
            blog_post_comment_rating,
        )))
    }

    pub fn find(
        &self,
        filter: GetAllBlogPostCommentRatingsFilter,
        sort: Option<BlogPostCommentRatingSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::BlogPostCommentRating>, diesel::result::Error> {
        use crate::schema::blog_post_comment_ratings::dsl::blog_post_comment_ratings;
        let q = blog_post_comment_ratings
            .select(blog_post_comment_ratings::all_columns())
            .into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = &self.conn.pg_conn;
        let results: Vec<db_models::BlogPostCommentRating> = q.load(conn)?;

        Ok(results
            .into_iter()
            .map(|blog_post_comment_rating| {
                domain::BlogPostCommentRating::from(blog_post_comment_rating)
            })
            .collect::<Vec<_>>())
    }
}
