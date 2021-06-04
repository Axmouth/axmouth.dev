use crate::filters::GetAllBlogPostsFilter;
use crate::models::{db_models, domain};
use crate::options::PaginationOptions;
use crate::schema::{blog_posts, blog_posts_categories, categories};
use crate::{
    change_sets::UpdateBlogPost,
    insertables::{NewBlogPost, NewBlogPostCategory, NewCategory},
};
use crate::{errors::PgRepoError, options::BlogPostSortType};

use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub struct BlogPostRepo<'a> {
    conn: &'a crate::pg_util::RepoConnection,
}

impl<'a> BlogPostRepo<'a> {
    pub fn new(conn: &'a crate::pg_util::RepoConnection) -> Self {
        Self { conn }
    }

    pub fn insert_one(&self, new_post: NewBlogPost) -> Result<usize, diesel::result::Error> {
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(blog_posts::table).values(new_post);
        Ok(query.execute(conn)?)
    }

    fn update_categories(
        &self,
        inserted_post_id: i32,
        categories_list: &Vec<String>,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::blog_posts_categories::dsl::{
            blog_post_id, blog_posts_categories as blog_posts_categories_dsl, category_id,
        };
        use crate::schema::categories::dsl::{categories as categories_dsl, name as category_name};
        let query =
            diesel::delete(blog_posts_categories::table).filter(blog_post_id.eq(inserted_post_id));
        let conn = &self.conn.pg_conn;
        let _ = query.execute(conn)?;
        let new_categories: Vec<NewCategory> = categories_list
            .clone()
            .into_iter()
            .map(|name| NewCategory { name })
            .collect();
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(categories::table)
            .values(&new_categories)
            .on_conflict_do_nothing();
        let _ = query.execute(conn)?;
        let query = categories_dsl
            .select(categories_dsl::all_columns())
            .into_boxed();
        let query = query.filter(category_name.eq_any(categories_list));
        let conn = &self.conn.pg_conn;
        let inserted_categories: Vec<db_models::Category> = query.load(conn)?;
        let new_blog_post_categories: Vec<NewBlogPostCategory> = inserted_categories
            .into_iter()
            .map(|category| NewBlogPostCategory {
                category_id: category.id,
                blog_post_id: inserted_post_id,
            })
            .collect();
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(blog_posts_categories::table)
            .values(&new_blog_post_categories)
            .on_conflict_do_nothing();
        let _ = query.execute(conn)?;
        Ok(())
    }

    pub fn insert_one_with_categories(
        &self,
        new_post: &NewBlogPost,
        categories_list: &Vec<String>,
    ) -> Result<i32, diesel::result::Error> {
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(blog_posts::table).values(new_post);
        let inserted_post: db_models::BlogPost = match query.get_result(conn).optional()? {
            None => return Err(diesel::result::Error::__Nonexhaustive),
            Some(value) => value,
        };
        let _ = self.update_categories(inserted_post.id, categories_list)?;

        Ok(inserted_post.id)
    }

    pub fn update_one(
        &self,
        id_value: i32,
        updated_post: &UpdateBlogPost,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::blog_posts::dsl::{blog_posts, id};
        let conn = &self.conn.pg_conn;
        let query = diesel::update(blog_posts.filter(id.eq(id_value))).set(updated_post);
        Ok(query.execute(conn)?)
    }

    pub fn update_one_with_categories(
        &self,
        id_value: i32,
        updated_post: &UpdateBlogPost,
        categories_list: &Vec<String>,
    ) -> Result<usize, diesel::result::Error> {
        let result = self.update_one(id_value, updated_post)?;
        let _ = self.update_categories(id_value, categories_list)?;
        Ok(result)
    }

    pub fn delete_one(&self, id_value: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::blog_post_comments::dsl::{
            blog_post_comments as blog_post_comments_dsl,
            post_id as blog_posts_comment_blog_post_id,
        };
        use crate::schema::blog_posts::dsl::{blog_posts, id};
        use crate::schema::blog_posts_categories::dsl::{
            blog_post_id as blog_posts_categories_blog_post_id,
            blog_posts_categories as blog_posts_categories_dsl,
        };
        let conn = &self.conn.pg_conn;
        let query = diesel::delete(
            blog_posts_categories_dsl.filter(blog_posts_categories_blog_post_id.eq(id_value)),
        );
        query.execute(conn)?;
        let conn = &self.conn.pg_conn;
        let query = diesel::delete(
            blog_post_comments_dsl.filter(blog_posts_comment_blog_post_id.eq(id_value)),
        );
        query.execute(conn)?;

        let conn = &self.conn.pg_conn;
        let query = diesel::delete(blog_posts.filter(id.eq(id_value)));
        Ok(query.execute(conn)?)
    }

    pub fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::BlogPost>, diesel::result::Error> {
        use crate::schema::blog_posts::dsl::{blog_posts as blog_posts_dsl, id as blog_post_id};
        use crate::schema::blog_posts_categories::dsl::{
            blog_post_id as blog_posts_categories_blog_post_id,
            blog_posts_categories as blog_posts_categories_dsl,
            category_id as blog_posts_categories_category_id,
        };
        use crate::schema::categories::dsl::{
            categories as categories_dsl, id as category_id, name as category_name,
        };
        use crate::schema::users::dsl::id as user_id;
        use crate::schema::users::dsl::users;

        let conn = &self.conn.pg_conn;

        let query = blog_posts_dsl
            .filter(blog_post_id.eq(id_value))
            .inner_join(users)
            .left_join(blog_posts_categories_dsl.inner_join(categories_dsl))
            .group_by((blog_post_id, user_id))
            .select((
                blog_posts_dsl::all_columns(),
                users::all_columns(),
                diesel::dsl::sql::<
                    diesel::sql_types::Array<
                        diesel::sql_types::Nullable<diesel::sql_types::VarChar>,
                    >,
                >("array_agg(\"categories\".\"name\")"),
            ));
        let (blog_post, user, categories_list): (
            db_models::BlogPost,
            db_models::User,
            Vec<Option<String>>,
        ) = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };
        Ok(Some(domain::BlogPost::from(
            blog_post,
            user,
            categories_list.into_iter().filter_map(|v| v).collect(),
        )))
    }

    pub fn find_one_by_slug(
        &self,
        slug_value: String,
    ) -> Result<Option<domain::BlogPost>, diesel::result::Error> {
        use crate::schema::blog_posts::dsl::{
            blog_posts as blog_posts_dsl, id as blog_post_id, slug,
        };
        use crate::schema::blog_posts_categories::dsl::{
            blog_post_id as blog_posts_categories_blog_post_id,
            blog_posts_categories as blog_posts_categories_dsl,
            category_id as blog_posts_categories_category_id,
        };
        use crate::schema::categories::dsl::{
            categories as categories_dsl, id as category_id, name as category_name,
        };
        use crate::schema::users::dsl::id as user_id;
        use crate::schema::users::dsl::users;

        let conn = &self.conn.pg_conn;

        let query = blog_posts_dsl
            .filter(slug.eq(slug_value))
            .inner_join(users)
            .left_join(blog_posts_categories_dsl.inner_join(categories_dsl))
            .group_by((blog_post_id, user_id))
            .select((
                blog_posts_dsl::all_columns(),
                users::all_columns(),
                diesel::dsl::sql::<
                    diesel::sql_types::Array<
                        diesel::sql_types::Nullable<diesel::sql_types::VarChar>,
                    >,
                >("array_agg(\"categories\".\"name\")"),
            ));
        let (blog_post, user, categories_list): (
            db_models::BlogPost,
            db_models::User,
            Vec<Option<String>>,
        ) = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };
        Ok(Some(domain::BlogPost::from(
            blog_post,
            user,
            categories_list.into_iter().filter_map(|v| v).collect(),
        )))
    }

    pub fn find(
        &self,
        filter: GetAllBlogPostsFilter,
        sort: Option<BlogPostSortType>,
        pagination: PaginationOptions,
    ) -> Result<(Vec<domain::BlogPost>, i64), diesel::result::Error> {
        use crate::schema::blog_posts::dsl::{
            blog_posts as blog_posts_dsl, id as blog_post_id, published as blog_post_published,
        };
        use crate::schema::blog_posts_categories::dsl::{
            blog_post_id as blog_posts_categories_blog_post_id,
            blog_posts_categories as blog_posts_categories_dsl,
            category_id as blog_posts_categories_category_id,
        };
        use crate::schema::categories::dsl::{
            categories as categories_dsl, id as category_id, name as category_name,
        };
        use crate::schema::users::dsl::{id as user_id, users};

        let q = blog_posts_dsl
            .inner_join(users)
            .left_join(blog_posts_categories_dsl.inner_join(categories_dsl))
            .group_by((blog_post_id, user_id))
            .select((
                blog_posts_dsl::all_columns(),
                users::all_columns(),
                diesel::dsl::sql::<
                    diesel::sql_types::Array<
                        diesel::sql_types::Nullable<diesel::sql_types::VarChar>,
                    >,
                >("array_agg(\"categories\".\"name\")"),
                diesel::dsl::sql::<diesel::sql_types::BigInt>("Count(*) Over()"),
            ))
            .into_boxed();

        let q = if let Some(a) = filter.author_id {
            q.filter(user_id.eq(a))
        } else {
            q
        };

        let q = if let Some(category_name_filter) = filter.category_name {
            q.filter(
                blog_post_id.eq_any(
                    blog_posts_categories_dsl
                        .inner_join(categories_dsl)
                        .filter(category_name.eq(category_name_filter.clone()))
                        .into_boxed()
                        .select(blog_posts_categories_blog_post_id),
                ),
            )
        } else {
            q
        };

        let q = if let Some(category_id_filter) = filter.category_id {
            q.filter(
                blog_post_id.eq_any(
                    blog_posts_categories_dsl
                        .filter(blog_posts_categories_category_id.eq(category_id_filter.clone()))
                        .into_boxed()
                        .select(blog_posts_categories_blog_post_id),
                ),
            )
        } else {
            q
        };

        let q = if let Some(published) = filter.published {
            q.filter(blog_post_published.eq(published))
        } else {
            q
        };
        let q = if let Some(sort_type) = sort {
            match sort_type {
                BlogPostSortType::CreatedAtAsc => q.order(blog_posts::created_at.asc()),
                BlogPostSortType::CreatedAtDesc => q.order(blog_posts::created_at.desc()),
                BlogPostSortType::TitleAsc => q.order(blog_posts::title.asc()),
                BlogPostSortType::TitleDesc => q.order(blog_posts::title.desc()),
            }
        } else {
            q
        };

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = &self.conn.pg_conn;
        let post_results: Vec<(
            db_models::BlogPost,
            db_models::User,
            Vec<Option<String>>,
            i64,
        )> = q.load(conn)?;

        let count = match post_results.get(0) {
            Some((_, _, _, value)) => *value,
            None => 0,
        };

        let blog_posts_list = post_results
            .into_iter()
            .map(|(post, user, categories_list, _)| {
                domain::BlogPost::from(
                    post,
                    user,
                    categories_list.into_iter().filter_map(|v| v).collect(),
                )
            })
            .collect::<Vec<_>>();
        Ok((blog_posts_list, count))
    }
}
