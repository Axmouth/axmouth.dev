use crate::errors::PgRepoError;
use crate::filters::GetAllBlogPostCommentFlagsFilter;
use crate::insertables::NewBlogPostCommentFlag;
use crate::models::{db_models, domain};
use crate::options::{BlogPostCommentFlagSort, PaginationOptions};
use crate::schema::blog_post_comment_flags;
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

#[derive(Clone)]
pub struct BlogPostCommentFlagRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl BlogPostCommentFlagRepo {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn insert_one(
        &self,
        new_blog_post_comment_flag: NewBlogPostCommentFlag,
    ) -> Result<domain::BlogPostCommentFlag, PgRepoError> {
        let conn = self.pool.get()?;
        let query =
            diesel::insert_into(blog_post_comment_flags::table).values(&new_blog_post_comment_flag);
        let result = tokio::task::block_in_place(move || query.get_result(&conn))?;
        Ok(domain::BlogPostCommentFlag::from(result))
    }

    pub async fn delete_one(&self, id_value: i32) -> Result<usize, PgRepoError> {
        use crate::schema::blog_post_comment_flags::dsl::{blog_post_comment_flags, id};
        let conn = self.pool.get()?;
        let query = diesel::delete(blog_post_comment_flags.filter(id.eq(id_value)));
        Ok(query.execute(&conn)?)
    }

    pub async fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::BlogPostCommentFlag>, PgRepoError> {
        use crate::schema::blog_post_comment_flags::dsl::{blog_post_comment_flags, id};

        let conn = self.pool.get()?;
        let query = blog_post_comment_flags
            .filter(id.eq(id_value))
            .select(blog_post_comment_flags::all_columns());
        let blog_post_comment_flag: db_models::BlogPostCommentFlag =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::BlogPostCommentFlag::from(
            blog_post_comment_flag,
        )))
    }

    pub async fn find(
        &self,
        filter: GetAllBlogPostCommentFlagsFilter,
        sort: BlogPostCommentFlagSort,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::BlogPostCommentFlag>, PgRepoError> {
        use crate::schema::blog_post_comment_flags::dsl::{blog_post_comment_flags, id};
        let q = blog_post_comment_flags
            .select(blog_post_comment_flags::all_columns())
            .into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = self.pool.get()?;
        let results: Vec<db_models::BlogPostCommentFlag> =
            tokio::task::block_in_place(move || q.load(&conn))?;

        Ok(results
            .into_iter()
            .map(|blog_post_comment_flag| domain::BlogPostCommentFlag::from(blog_post_comment_flag))
            .collect::<Vec<_>>())
    }
}
