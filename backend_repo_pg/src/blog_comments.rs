use crate::filters::GetAllBlogPostCommentsFilter;
use crate::models::{db_models, domain};
use crate::options::PaginationOptions;
use crate::schema::blog_post_comments;
use crate::{change_sets::UpdateBlogPostComment, insertables::NewBlogPostComment};
use crate::{errors::PgRepoError, options::BlogPostCommentSortType};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub struct BlogPostCommentRepo<'a> {
    conn: &'a crate::pg_util::RepoConnection,
}

impl<'a> BlogPostCommentRepo<'a> {
    pub fn new(conn: &'a crate::pg_util::RepoConnection) -> Self {
        Self { conn }
    }

    pub fn insert_one(
        &self,
        new_comment: NewBlogPostComment,
    ) -> Result<usize, diesel::result::Error> {
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(blog_post_comments::table).values(&new_comment);
        Ok(query.execute(conn)?)
    }

    pub fn update_one(
        &self,
        id_value: i32,
        updated_comment: UpdateBlogPostComment,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::blog_post_comments::dsl::{blog_post_comments, id};
        let conn = &self.conn.pg_conn;
        let query =
            diesel::update(blog_post_comments.filter(id.eq(id_value))).set(&updated_comment);
        Ok(query.execute(conn)?)
    }

    pub fn delete_one(&self, id_value: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::blog_post_comments::dsl::{blog_post_comments, id};
        let conn = &self.conn.pg_conn;
        let query = diesel::delete(blog_post_comments.filter(id.eq(id_value)));
        Ok(query.execute(conn)?)
    }

    pub fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::BlogPostComment>, diesel::result::Error> {
        use crate::schema::blog_post_comments::dsl::{blog_post_comments, id};
        use crate::schema::users::dsl::users;

        let conn = &self.conn.pg_conn;
        let query = blog_post_comments
            .filter(id.eq(id_value))
            .inner_join(users)
            .select((blog_post_comments::all_columns(), users::all_columns()));
        let (blog_post_comment, user): (db_models::BlogPostComment, db_models::User) =
            match query.first(conn).optional()? {
                Some(value) => value,
                None => return Ok(None),
            };

        Ok(Some(domain::BlogPostComment::from(blog_post_comment, user)))
    }

    pub fn find(
        &self,
        filter: GetAllBlogPostCommentsFilter,
        sort: Option<BlogPostCommentSortType>,
        pagination: PaginationOptions,
    ) -> Result<(Vec<domain::BlogPostComment>, i64), diesel::result::Error> {
        use crate::schema::blog_post_comments::dsl::{
            blog_post_comments as blog_post_comments_dsl, post_id,
        };
        use crate::schema::users::dsl::{id as user_id, users};
        let q = blog_post_comments_dsl
            .inner_join(users)
            .select((
                blog_post_comments_dsl::all_columns(),
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

        let q = if let Some(sort_type) = sort {
            match sort_type {
                BlogPostCommentSortType::CreatedAtAsc => {
                    q.order(blog_post_comments::created_at.asc())
                }
                BlogPostCommentSortType::CreatedAtDesc => {
                    q.order(blog_post_comments::created_at.desc())
                }
            }
        } else {
            q
        };

        let conn = &self.conn.pg_conn;
        let results: Vec<(db_models::BlogPostComment, db_models::User, i64)> = q.load(conn)?;

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
