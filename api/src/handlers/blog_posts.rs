use crate::errors::AppError;
use crate::extractors::AdminClaimsContext;
use crate::extractors::OptClaimsContext;
use crate::extractors::ValidatedJson;
use crate::util::bad_request_response;
use crate::util::create_creation_admin_log;
use crate::util::create_deletion_admin_log;
use crate::util::create_update_admin_log;
use crate::util::{
    not_found_response, paginated_ok_response, simple_created_response, simple_no_content_response,
    simple_ok_response,
};
use axum::extract::Extension;
use axum::extract::Path;
use axum::response::IntoResponse;
use backend_repo_pg::blog_posts::BlogPostRepo;
use backend_repo_pg::errors::PgRepoError;
use backend_repo_pg::models::queries::GetBlogPostQuery;
use backend_repo_pg::models::queries::PaginatedQuery;
use backend_repo_pg::pg_util::get_roll_back_err;
use backend_repo_pg::pg_util::pg_transaction;
use backend_repo_pg::pg_util::DynRepo;
use backend_repo_pg::{
    change_sets::UpdateBlogPost,
    filters::GetAllBlogPostsFilter,
    insertables::NewBlogPost,
    models::{
        queries::GetAllBlogPostsQuery,
        requests::{CreateBlogPostRequest, UpdateBlogPostRequest},
    },
};
use chrono::Utc;
use tokio::task::block_in_place;

pub async fn get(
    Path(id): Path<String>,
    GetBlogPostQuery { use_slug }: GetBlogPostQuery,
    OptClaimsContext { claims }: OptClaimsContext,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    block_in_place(|| {
        let conn = repo.get_conn()?;
        let blog_post_repository = BlogPostRepo::new(&conn);
        let post_result = if let Some(true) = use_slug {
            match blog_post_repository
                .find_one_by_slug(id)
                .map_err::<PgRepoError, _>(|e| e.into())?
            {
                None => {
                    return Ok(not_found_response("Post"));
                }
                Some(value) => value,
            }
        } else {
            let id = match id.parse::<i32>() {
                Ok(v) => v,
                Err(_) => {
                    return Ok(bad_request_response("Url: Bad Id value"));
                }
            };
            match blog_post_repository
                .find_one(id)
                .map_err::<PgRepoError, _>(|e| e.into())?
            {
                None => {
                    return Ok(not_found_response("Post"));
                }
                Some(value) => value,
            }
        };
        if let Some(claims) = claims {
            if !claims.is_staff() && !post_result.published {
                return Ok(not_found_response("Post"));
            }
        }
        Ok(simple_ok_response(post_result))
    })
}

pub async fn get_all(
    query: GetAllBlogPostsQuery,
    OptClaimsContext { claims }: OptClaimsContext,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    block_in_place(|| {
        let conn = repo.get_conn()?;
        let mut filter = GetAllBlogPostsFilter::from_query(query.clone());
        if let Some(claims) = claims {
            if !claims.is_staff() {
                filter.published = Some(true);
            }
        } else {
            filter.published = Some(true);
        }
        let blog_post_repository = BlogPostRepo::new(&conn);
        let pagination_opts = query.pagination_options();
        let sort_type = query.sort_type;
        let (posts_list, total_results) = blog_post_repository
            .find(filter, sort_type, pagination_opts)
            .map_err::<PgRepoError, _>(|e| e.into())?;
        Ok(paginated_ok_response(
            posts_list,
            query.page,
            query.page_size,
            total_results,
        ))
    })
}

pub async fn delete(
    Path(id): Path<i32>,
    AdminClaimsContext { claims }: AdminClaimsContext,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    Ok(pg_transaction(repo, |conn| {
        let blog_post_repository = BlogPostRepo::new(conn);
        let old_data = match blog_post_repository.find_one(id)? {
            None => {
                return Ok(not_found_response("Post"));
            }
            Some(value) => value,
        };
        let post_result = blog_post_repository.delete_one(id)?;
        if post_result == 0 {
            return Ok(not_found_response("Post"));
        }
        match create_deletion_admin_log(
            id.to_string(),
            claims.user_id(),
            String::from("Blog Post"),
            String::from("blog_posts"),
            &old_data,
            String::from("/blog-posts"),
            conn,
        ) {
            Ok(_) => {}
            Err(_) => {
                return Err(get_roll_back_err());
            }
        };
        Ok(simple_no_content_response(post_result))
    })
    .await?)
}

pub async fn update(
    Path(id): Path<i32>,
    AdminClaimsContext { claims }: AdminClaimsContext,
    ValidatedJson(request): ValidatedJson<UpdateBlogPostRequest>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    Ok(pg_transaction(repo, |conn| {
        let blog_post_repository = BlogPostRepo::new(conn);
        let old_data = match blog_post_repository.find_one(id)? {
            Some(value) => value,
            None => {
                return Ok(not_found_response("Post"));
            }
        };
        let post_updates = UpdateBlogPost {
            title: request.title,
            body: request.body,
            published: request.published,
            updated_at: Some(Some(Utc::now().naive_utc())),
            description: request.description,
            slug: request.slug,
        };
        let post_result = if let Some(categories_list) = request.categories {
            blog_post_repository.update_one_with_categories(id, &post_updates, &categories_list)?
        } else {
            blog_post_repository.update_one(id, &post_updates)?
        };
        match create_update_admin_log(
            id.to_string(),
            claims.user_id(),
            String::from("Blog Post"),
            String::from("blog_posts"),
            &post_updates,
            &old_data,
            String::from("/blog-posts"),
            conn,
        ) {
            Ok(_) => {}
            Err(_) => {
                return Err(get_roll_back_err());
            }
        };
        Ok(simple_created_response(post_result))
    })
    .await?)
}

pub async fn create(
    AdminClaimsContext { claims }: AdminClaimsContext,
    ValidatedJson(request): ValidatedJson<CreateBlogPostRequest>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    Ok(pg_transaction(repo, |conn| {
        let new_post = NewBlogPost {
            title: request.title,
            body: request.body,
            author_id: claims.user_id(),
            published: false,
            description: request.description,
            slug: request.slug,
        };
        let new_post_copy = new_post.clone();
        let blog_post_repository = BlogPostRepo::new(conn);
        let post_result =
            blog_post_repository.insert_one_with_categories(&new_post, &request.categories)?;
        match create_creation_admin_log(
            post_result.to_string(),
            claims.user_id(),
            String::from("Blog Post"),
            String::from("blog_posts"),
            &new_post_copy,
            String::from("/blog-posts"),
            conn,
        ) {
            Ok(_) => {}
            Err(_) => {
                return Err(get_roll_back_err());
            }
        };
        Ok(simple_created_response(post_result))
    })
    .await?)
}
