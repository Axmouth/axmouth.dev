use crate::errors::AppError;
use crate::extractors::ClaimsContext;
use crate::extractors::ValidatedJson;
use crate::util::create_deletion_admin_log;
use crate::util::create_update_admin_log;
use crate::util::{
    bad_request_response, not_found_response, paginated_ok_response, simple_created_response,
    simple_no_content_response, simple_ok_response, unauthorized_response,
};
use axum::extract::Extension;
use axum::extract::Path;
use axum::response::IntoResponse;
use backend_repo_pg::errors::PgRepoError;
use backend_repo_pg::models::queries::PaginatedQuery;
use backend_repo_pg::pg_util::get_roll_back_err;
use backend_repo_pg::pg_util::pg_transaction;
use backend_repo_pg::pg_util::DynRepo;
use backend_repo_pg::{blog_comments::BlogPostCommentRepo, blog_posts::BlogPostRepo};
use backend_repo_pg::{
    change_sets::UpdateBlogPostComment,
    filters::GetAllBlogPostCommentsFilter,
    insertables::NewBlogPostComment,
    models::{
        queries::GetAllBlogPostCommentsQuery,
        requests::{CreateBlogPostCommentRequest, UpdateBlogPostCommentRequest},
    },
};
use chrono::Utc;
use tokio::task::block_in_place;

pub async fn get(
    Path(id): Path<i32>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    block_in_place(|| {
        let conn = repo.get_conn()?;
        let blog_comment_repository = BlogPostCommentRepo::new(&conn);
        let comment_result = match blog_comment_repository
            .find_one(id)
            .map_err::<PgRepoError, _>(|e| e.into())?
        {
            None => {
                return Ok(not_found_response("Comment"));
            }
            Some(value) => value,
        };
        Ok(simple_ok_response(comment_result))
    })
}

pub async fn get_all(
    query: GetAllBlogPostCommentsQuery,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    block_in_place(|| {
        let conn = repo.get_conn()?;
        let filter = GetAllBlogPostCommentsFilter::from_query(query.clone());
        let blog_comment_repository = BlogPostCommentRepo::new(&conn);
        let pagination_opts = query.pagination_options();
        let sort_type = query.sort_type;
        let (comments_list, total_results) = blog_comment_repository
            .find(filter, sort_type, pagination_opts)
            .map_err::<PgRepoError, _>(|e| e.into())?;
        Ok(paginated_ok_response(
            comments_list,
            query.page,
            query.page_size,
            total_results,
        ))
    })
}

pub async fn delete(
    Path(id): Path<i32>,
    ClaimsContext { claims }: ClaimsContext,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    Ok(pg_transaction(repo, |conn| {
        let blog_comment_repository = BlogPostCommentRepo::new(conn);
        let comment = match blog_comment_repository.find_one(id)? {
            None => {
                return Ok(not_found_response("Comment"));
            }
            Some(value) => value,
        };
        if comment.author.id != claims.user_id() || !claims.is_admin() {
            return Ok(unauthorized_response("comment"));
        }
        let comment_result = blog_comment_repository.delete_one(id)?;
        if comment_result == 0 {
            return Ok(not_found_response("Comment"));
        }
        if claims.is_for_admin_site() {
            match create_deletion_admin_log(
                id.to_string(),
                claims.user_id(),
                String::from("Blog Post Comment"),
                String::from("blog_post_comments"),
                &comment,
                String::from("/blog-post-comments"),
                conn,
            ) {
                Ok(_) => {}
                Err(_) => {
                    return Err(get_roll_back_err());
                }
            };
        }
        Ok(simple_no_content_response(comment_result))
    })
    .await?)
}

pub async fn update(
    Path(id): Path<i32>,
    ClaimsContext { claims }: ClaimsContext,
    ValidatedJson(request): ValidatedJson<UpdateBlogPostCommentRequest>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    Ok(pg_transaction(repo, |conn| {
        let blog_comment_repository = BlogPostCommentRepo::new(conn);
        let request_copy = request.clone();
        let comment = match blog_comment_repository.find_one(id)? {
            Some(value) => value,
            None => {
                return Ok(not_found_response("Comment"));
            }
        };
        if comment.author.id != claims.user_id() || !claims.is_admin() {
            return Ok(not_found_response("Comment"));
        }
        let comment_updates = UpdateBlogPostComment {
            body: Some(request.body),
            updated_at: Some(Some(Utc::now().naive_utc())),
        };
        let comment_result = blog_comment_repository.update_one(id, comment_updates)?;
        if claims.is_for_admin_site() {
            match create_update_admin_log(
                id.to_string(),
                claims.user_id(),
                String::from("Blog Post Comment"),
                String::from("blog_post_comments"),
                &request_copy,
                &comment,
                String::from("/blog-post-comments"),
                conn,
            ) {
                Ok(_) => {}
                Err(_) => {
                    return Err(get_roll_back_err());
                }
            };
        }
        Ok(simple_created_response(comment_result))
    })
    .await?)
}

pub async fn create(
    ClaimsContext { claims }: ClaimsContext,
    ValidatedJson(request): ValidatedJson<CreateBlogPostCommentRequest>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    Ok(pg_transaction(repo, |conn| {
        let blog_post_repository = BlogPostRepo::new(conn);
        let _ = match blog_post_repository.find_one(request.post_id)? {
            None => {
                return Ok(bad_request_response("Invalid post id"));
            }
            Some(value) => value,
        };
        let new_comment = NewBlogPostComment {
            body: Some(request.body),
            post_id: request.post_id,
            author_id: claims.user_id(),
        };
        let blog_comment_repository = BlogPostCommentRepo::new(conn);
        let comment_result = blog_comment_repository.insert_one(new_comment)?;
        Ok(simple_no_content_response(comment_result))
    })
    .await?)
}
