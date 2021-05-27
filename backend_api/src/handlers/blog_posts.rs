use crate::app::AppState;
use crate::util::bad_request_response;
use crate::util::create_creation_admin_log;
use crate::util::create_deletion_admin_log;
use crate::util::create_update_admin_log;
use crate::{
    auth_tokens,
    util::{
        not_found_response, paginated_ok_response, server_error_response, simple_created_response,
        simple_no_content_response, simple_ok_response,
    },
};
use auth_tokens::Claims;
use backend_repo_pg::models::queries::GetBlogPostQuery;
use backend_repo_pg::{blog_posts::BlogPostRepo, options::PaginationOptions};
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

pub async fn get(
    id: String,
    query: GetBlogPostQuery,
    claims: Option<Claims>,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let blog_post_repository = BlogPostRepo::new(state.repo.clone());
    let post_result = if let Some(true) = query.use_slug {
        match blog_post_repository.find_one_by_slug(id).await {
            Err(err) => {
                return Ok(server_error_response(err));
            }
            Ok(value_opt) => match value_opt {
                None => {
                    return Ok(not_found_response("Post"));
                }
                Some(value) => value,
            },
        }
    } else {
        let id = match id.parse::<i32>() {
            Ok(v) => v,
            Err(_) => {
                return Ok(bad_request_response("Url: Bad Id value"));
            }
        };
        match blog_post_repository.find_one(id).await {
            Err(err) => {
                return Ok(server_error_response(err));
            }
            Ok(value_opt) => match value_opt {
                None => {
                    return Ok(not_found_response("Post"));
                }
                Some(value) => value,
            },
        }
    };
    if let Some(claims) = claims {
        if claims.is_staff() == false && post_result.published == false {
            return Ok(not_found_response("Post"));
        }
    }
    Ok(simple_ok_response(post_result))
}

pub async fn get_all(
    query: GetAllBlogPostsQuery,
    claims: Option<Claims>,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut filter = GetAllBlogPostsFilter::from_query(query.clone());
    if let Some(claims) = claims {
        if claims.is_staff() == false {
            filter.published = Some(true);
        }
    } else {
        filter.published = Some(true);
    }
    let blog_post_repository = BlogPostRepo::new(state.repo.clone());
    let (posts_list, total_results) = match blog_post_repository
        .find(
            filter,
            query.sort_type,
            PaginationOptions {
                page: query.page,
                page_size: query.page_size,
            },
        )
        .await
    {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    Ok(paginated_ok_response(
        posts_list,
        query.page,
        query.page_size,
        total_results,
    ))
}

pub async fn delete(
    id: i32,
    claims: Claims,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let blog_post_repository = BlogPostRepo::new(state.repo.clone());
    let old_data = match blog_post_repository.find_one(id).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value_opt) => match value_opt {
            None => {
                return Ok(not_found_response("Post"));
            }
            Some(value) => value,
        },
    };
    let post_result = match blog_post_repository.delete_one(id).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    if post_result == 0 {
        return Ok(not_found_response("Post"));
    }
    match create_deletion_admin_log(
        id.to_string(),
        claims.user_id(),
        String::from("Blog Post"),
        String::from("blog_posts"),
        &old_data,
        String::from("/api/v1/blog-posts"),
        state.repo.clone(),
    )
    .await
    {
        Ok(_) => {}
        Err(err) => {
            return Ok(server_error_response(err));
        }
    };
    Ok(simple_no_content_response(post_result))
}

pub async fn update(
    id: i32,
    claims: Claims,
    request: UpdateBlogPostRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let blog_post_repository = BlogPostRepo::new(state.repo.clone());
    let old_data = match blog_post_repository.find_one(id).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value_opt) => match value_opt {
            Some(value) => value,
            None => {
                return Ok(not_found_response("Post"));
            }
        },
    };
    let post_updates = UpdateBlogPost {
        body: request.body,
        published: request.published,
        updated_at: Some(Some(Utc::now().naive_utc())),
        description: request.description,
        slug: request.slug,
    };
    let post_result = if let Some(categories_list) = request.categories {
        match blog_post_repository
            .update_one_with_categories(id, &post_updates, &categories_list)
            .await
        {
            Err(err) => {
                return Ok(server_error_response(err));
            }
            Ok(value) => value,
        }
    } else {
        match blog_post_repository.update_one(id, &post_updates).await {
            Err(err) => {
                return Ok(server_error_response(err));
            }
            Ok(value) => value,
        }
    };
    match create_update_admin_log(
        id.to_string(),
        claims.user_id(),
        String::from("Blog Post"),
        String::from("blog_posts"),
        &post_updates,
        &old_data,
        String::from("/api/v1/blog-posts"),
        state.repo.clone(),
    )
    .await
    {
        Ok(_) => {}
        Err(err) => {
            return Ok(server_error_response(err));
        }
    };
    Ok(simple_created_response(post_result))
}

pub async fn create(
    claims: Claims,
    request: CreateBlogPostRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let new_post = NewBlogPost {
        title: request.title,
        body: request.body,
        author_id: claims.user_id(),
        published: false,
        description: request.description,
        slug: request.slug,
    };
    let new_post_copy = new_post.clone();
    let blog_post_repository = BlogPostRepo::new(state.repo.clone());
    let post_result = match blog_post_repository
        .insert_one_with_categories(&new_post, &request.categories)
        .await
    {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    match create_creation_admin_log(
        post_result.to_string(),
        claims.user_id(),
        String::from("Blog Post"),
        String::from("blog_posts"),
        &new_post_copy,
        String::from("/api/v1/blog-posts"),
        state.repo.clone(),
    )
    .await
    {
        Ok(_) => {}
        Err(err) => {
            return Ok(server_error_response(err));
        }
    };
    Ok(simple_created_response(post_result))
}
