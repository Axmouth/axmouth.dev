use crate::app::AppState;
use crate::{
    auth_tokens,
    util::{
        not_found_response, paginated_ok_response, server_error_response, simple_created_response,
        simple_no_content_response, simple_ok_response,
    },
};
use auth_tokens::Claims;
use backend_repo_pg::{categories::CategoryRepo, options::PaginationOptions};
use backend_repo_pg::{
    change_sets::UpdateCategory,
    filters::GetAllCategoriesFilter,
    insertables::NewCategory,
    models::{
        queries::GetAllCategoriesQuery,
        requests::{CreateCategoryRequest, UpdateCategoryRequest},
    },
};

pub async fn get(id: i32, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    let category_repository = CategoryRepo::new(state.repo.clone());
    let category_result = match category_repository.find_one(id).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value_opt) => match value_opt {
            None => {
                return Ok(not_found_response("Category"));
            }
            Some(value) => value,
        },
    };
    Ok(simple_ok_response(category_result))
}

pub async fn get_all(
    query: GetAllCategoriesQuery,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let filter = GetAllCategoriesFilter::from_query(query.clone());
    let category_repository = CategoryRepo::new(state.repo.clone());
    let (categories_list, total_results) = match category_repository
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
        categories_list,
        query.page,
        query.page_size,
        total_results,
    ))
}

pub async fn delete(
    id: i32,
    _claims: Claims,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let category_repository = CategoryRepo::new(state.repo.clone());
    let _ = match category_repository.find_one(id).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value_opt) => match value_opt {
            None => {
                return Ok(not_found_response("Category"));
            }
            Some(value) => value,
        },
    };
    let category_result = match category_repository.delete_one(id).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    if category_result == 0 {
        return Ok(not_found_response("Category"));
    }
    Ok(simple_no_content_response(category_result))
}

pub async fn update(
    id: i32,
    _claims: Claims,
    request: UpdateCategoryRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let category_repository = CategoryRepo::new(state.repo.clone());
    let _ = match category_repository.find_one(id).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value_opt) => match value_opt {
            Some(value) => value,
            None => {
                return Ok(not_found_response("Link"));
            }
        },
    };
    let updated_category = UpdateCategory { name: request.name };
    let category_result = match category_repository.update_one(id, updated_category).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    Ok(simple_created_response(category_result))
}

pub async fn create(
    _claims: Claims,
    request: CreateCategoryRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let new_category = NewCategory { name: request.name };
    let category_repository = CategoryRepo::new(state.repo.clone());
    let category_result = match category_repository.insert_one(new_category).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    Ok(simple_created_response(category_result))
}
