use crate::app::AppState;
use crate::util::create_deletion_admin_log;
use crate::util::create_update_admin_log;
use crate::{
    auth_tokens::Claims,
    util::{
        bad_request_response, not_found_response, paginated_ok_response, simple_created_response,
        simple_no_content_response, simple_ok_response, unauthorized_response,
    },
};
use backend_repo_pg::pg_util::get_roll_back_err;
use backend_repo_pg::{
    blog_comments::BlogPostCommentRepo, blog_posts::BlogPostRepo, options::PaginationOptions,
};
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

pub async fn get(id: i32, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let blog_comment_repository = BlogPostCommentRepo::new(&conn);
            let comment_result = match blog_comment_repository.find_one(id)? {
                None => {
                    return Ok(not_found_response("Comment"));
                }
                Some(value) => value,
            };
            Ok(simple_ok_response(comment_result))
        })
        .await?)
}

pub async fn get_all(
    query: GetAllBlogPostCommentsQuery,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let filter = GetAllBlogPostCommentsFilter::from_query(query.clone());
            let blog_comment_repository = BlogPostCommentRepo::new(&conn);
            let (comments_list, total_results) = blog_comment_repository.find(
                filter,
                query.sort_type,
                PaginationOptions {
                    page: query.page,
                    page_size: query.page_size,
                },
            )?;
            Ok(paginated_ok_response(
                comments_list,
                query.page,
                query.page_size,
                total_results,
            ))
        })
        .await?)
}

pub async fn delete(
    id: i32,
    claims: Claims,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let blog_comment_repository = BlogPostCommentRepo::new(&conn);
            let comment = match blog_comment_repository.find_one(id)? {
                None => {
                    return Ok(not_found_response("Comment"));
                }
                Some(value) => value,
            };
            if comment.author.id != claims.user_id() || claims.is_admin() == false {
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
                    state.repo.clone(),
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
    id: i32,
    claims: Claims,
    request: UpdateBlogPostCommentRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let blog_comment_repository = BlogPostCommentRepo::new(&conn);
            let request_copy = request.clone();
            let comment = match blog_comment_repository.find_one(id)? {
                Some(value) => value,
                None => {
                    return Ok(not_found_response("Comment"));
                }
            };
            if comment.author.id != claims.user_id() || claims.is_admin() == false {
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
                    state.repo.clone(),
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
    claims: Claims,
    request: CreateBlogPostCommentRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let blog_post_repository = BlogPostRepo::new(&conn);
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
            let blog_comment_repository = BlogPostCommentRepo::new(&conn);
            let comment_result = blog_comment_repository.insert_one(new_comment)?;
            Ok(simple_no_content_response(comment_result))
        })
        .await?)
}
