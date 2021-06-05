use crate::errors::PgRepoError;
use crate::filters::GetAllBlogPostCommentFlagsFilter;
use crate::insertables::NewBlogPostCommentFlag;
use crate::models::{db_models, domain};
use crate::options::{BlogPostCommentFlagSortType, PaginationOptions};
use crate::schema::blog_post_comment_flags;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub struct BlogPostCommentFlagRepo<'a> {
    conn: &'a crate::pg_util::RepoConnection,
}

impl<'a> BlogPostCommentFlagRepo<'a> {
    pub fn new(conn: &'a crate::pg_util::RepoConnection) -> Self {
        Self { conn }
    }

    pub fn insert_one(
        &self,
        new_blog_post_comment_flag: NewBlogPostCommentFlag,
    ) -> Result<domain::BlogPostCommentFlag, diesel::result::Error> {
        let conn = &self.conn.pg_conn;
        let query =
            diesel::insert_into(blog_post_comment_flags::table).values(&new_blog_post_comment_flag);
        let result = query.get_result(conn)?;
        Ok(domain::BlogPostCommentFlag::from(result))
    }

    pub fn delete_one(&self, id_value: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::blog_post_comment_flags::dsl::{blog_post_comment_flags, id};
        let conn = &self.conn.pg_conn;
        let query = diesel::delete(blog_post_comment_flags.filter(id.eq(id_value)));
        Ok(query.execute(conn)?)
    }

    pub fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::BlogPostCommentFlag>, diesel::result::Error> {
        use crate::schema::blog_post_comment_flags::dsl::{blog_post_comment_flags, id};

        let conn = &self.conn.pg_conn;
        let query = blog_post_comment_flags
            .filter(id.eq(id_value))
            .select(blog_post_comment_flags::all_columns());
        let blog_post_comment_flag: db_models::BlogPostCommentFlag =
            match query.first(conn).optional()? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::BlogPostCommentFlag::from(
            blog_post_comment_flag,
        )))
    }

    pub fn find(
        &self,
        filter: GetAllBlogPostCommentFlagsFilter,
        sort: Option<BlogPostCommentFlagSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::BlogPostCommentFlag>, diesel::result::Error> {
        use crate::schema::blog_post_comment_flags::dsl::blog_post_comment_flags;
        let q = blog_post_comment_flags
            .select(blog_post_comment_flags::all_columns())
            .into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = &self.conn.pg_conn;
        let results: Vec<db_models::BlogPostCommentFlag> = q.load(conn)?;

        Ok(results
            .into_iter()
            .map(|blog_post_comment_flag| domain::BlogPostCommentFlag::from(blog_post_comment_flag))
            .collect::<Vec<_>>())
    }
}
