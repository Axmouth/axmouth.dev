use crate::errors::PgRepoError;
use crate::filters::GetAllBlogPostCommentsFilter;
use crate::models::{db_models, domain};
use crate::options::{BlogPostCommentSort, PaginationOptions};
use crate::schema::blog_post_comments;
use crate::{change_sets::UpdateBlogPostComment, insertables::NewBlogPostComment};
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

#[derive(Clone)]
pub struct BlogPostCommentRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl BlogPostCommentRepo {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn insert_one(&self, new_comment: NewBlogPostComment) -> Result<usize, PgRepoError> {
        let conn = self.pool.get()?;
        let query = diesel::insert_into(blog_post_comments::table).values(&new_comment);
        Ok(tokio::task::block_in_place(move || query.execute(&conn))?)
    }

    pub async fn update_one(
        &self,
        id_value: i32,
        updated_comment: UpdateBlogPostComment,
    ) -> Result<usize, PgRepoError> {
        use crate::schema::blog_post_comments::dsl::{blog_post_comments, id};
        let conn = self.pool.get()?;
        let query =
            diesel::update(blog_post_comments.filter(id.eq(id_value))).set(&updated_comment);
        Ok(tokio::task::block_in_place(move || query.execute(&conn))?)
    }

    pub async fn delete_one(&self, id_value: i32) -> Result<usize, PgRepoError> {
        use crate::schema::blog_post_comments::dsl::{blog_post_comments, id};
        let conn = self.pool.get()?;
        let query = diesel::delete(blog_post_comments.filter(id.eq(id_value)));
        Ok(tokio::task::block_in_place(move || query.execute(&conn))?)
    }

    pub async fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::BlogPostComment>, PgRepoError> {
        use crate::schema::blog_post_comments::dsl::{blog_post_comments, id};
        use crate::schema::users::dsl::users;

        let conn = self.pool.get()?;
        let query = blog_post_comments
            .filter(id.eq(id_value))
            .inner_join(users)
            .select((blog_post_comments::all_columns(), users::all_columns()));
        let (blog_post_comment, user): (db_models::BlogPostComment, db_models::User) =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };

        Ok(Some(domain::BlogPostComment::from(blog_post_comment, user)))
    }

    pub async fn find(
        &self,
        filter: GetAllBlogPostCommentsFilter,
        sort: BlogPostCommentSort,
        pagination: PaginationOptions,
    ) -> Result<(Vec<domain::BlogPostComment>, i64), PgRepoError> {
        use crate::schema::blog_post_comments::dsl::{blog_post_comments, id, post_id};
        use crate::schema::users::dsl::{id as user_id, users};
        use diesel::dsl::count_star;
        let q = blog_post_comments
            .inner_join(users)
            .select((
                blog_post_comments::all_columns(),
                users::all_columns(),
                diesel::dsl::sql::<diesel::sql_types::BigInt>("count(*) over()"),
            ))
            .into_boxed();

        let q = if let Some(a) = filter.author_id {
            q.filter(user_id.eq(a))
        } else {
            q
        };

        let q = if let Some(a) = filter.post_id {
            q.filter(post_id.eq(a))
        } else {
            q
        };

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = self.pool.get()?;
        let results: Vec<(db_models::BlogPostComment, db_models::User, i64)> =
            tokio::task::block_in_place(move || q.load(&conn))?;

        let count = match results.get(0) {
            Some((_, _, value)) => *value,
            None => 0,
        };
        let blog_post_comments_list = results
            .into_iter()
            .map(|(comment, user, _)| domain::BlogPostComment::from(comment, user))
            .collect::<Vec<_>>();
        Ok((blog_post_comments_list, count))
    }
}
