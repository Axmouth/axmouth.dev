use crate::errors::AppError;
use crate::extractors::AdminClaimsContext;
use crate::extractors::ValidatedJson;
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
use backend_repo_pg::categories::CategoryRepo;
use backend_repo_pg::errors::PgRepoError;
use backend_repo_pg::models::queries::PaginatedQuery;
use backend_repo_pg::pg_util::get_roll_back_err;
use backend_repo_pg::pg_util::pg_transaction;
use backend_repo_pg::pg_util::DynRepo;
use backend_repo_pg::{
    change_sets::UpdateCategory,
    filters::GetAllCategoriesFilter,
    insertables::NewCategory,
    models::{
        queries::GetAllCategoriesQuery,
        requests::{CreateCategoryRequest, UpdateCategoryRequest},
    },
};
use tokio::task::block_in_place;

pub async fn get(
    Path(id): Path<i32>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    block_in_place(|| {
        let conn = repo.get_conn()?;
        let category_repository = CategoryRepo::new(&conn);
        let category_result = match category_repository
            .find_one(id)
            .map_err::<PgRepoError, _>(|e| e.into())?
        {
            None => {
                return Ok(not_found_response("Category"));
            }
            Some(value) => value,
        };
        Ok(simple_ok_response(category_result))
    })
}

pub async fn get_all(
    query: GetAllCategoriesQuery,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    block_in_place(|| {
        let conn = repo.get_conn()?;
        let filter = GetAllCategoriesFilter::from_query(query.clone());
        let category_repository = CategoryRepo::new(&conn);
        let pagination_opts = query.pagination_options();
        let sort_type = query.sort_type;
        let (categories_list, total_results) = category_repository
            .find(filter, sort_type, pagination_opts)
            .map_err::<PgRepoError, _>(|e| e.into())?;
        Ok(paginated_ok_response(
            categories_list,
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
        let category_repository = CategoryRepo::new(conn);
        let old_data = match category_repository.find_one(id)? {
            None => {
                return Ok(not_found_response("Category"));
            }
            Some(value) => value,
        };
        let category_result = category_repository.delete_one(id)?;
        if category_result == 0 {
            return Ok(not_found_response("Category"));
        }
        match create_deletion_admin_log(
            id.to_string(),
            claims.user_id(),
            String::from("Blog Post Category"),
            String::from("categories"),
            &old_data,
            String::from("/categories"),
            conn,
        ) {
            Ok(_) => {}
            Err(_) => {
                return Err(get_roll_back_err());
            }
        };
        Ok(simple_no_content_response(category_result))
    })
    .await?)
}

pub async fn update(
    Path(id): Path<i32>,
    AdminClaimsContext { claims }: AdminClaimsContext,
    ValidatedJson(request): ValidatedJson<UpdateCategoryRequest>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    Ok(pg_transaction(repo.clone(), |conn| {
        let category_repository = CategoryRepo::new(conn);
        let request_copy = request.clone();
        let old_data = match category_repository.find_one(id)? {
            Some(value) => value,
            None => {
                return Ok(not_found_response("Link"));
            }
        };
        let updated_category = UpdateCategory { name: request.name };
        let category_result = category_repository.update_one(id, &updated_category)?;
        match create_update_admin_log(
            id.to_string(),
            claims.user_id(),
            String::from("Blog Post Category"),
            String::from("categories"),
            &request_copy,
            &old_data,
            String::from("/categories"),
            conn,
        ) {
            Ok(_) => {}
            Err(_) => {
                return Err(get_roll_back_err());
            }
        };
        Ok(simple_created_response(category_result))
    })
    .await?)
}

pub async fn create(
    AdminClaimsContext { claims }: AdminClaimsContext,
    ValidatedJson(request): ValidatedJson<CreateCategoryRequest>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    Ok(pg_transaction(repo.clone(), |conn| {
        let new_category = NewCategory { name: request.name };
        let new_category_copy = new_category.clone();
        let category_repository = CategoryRepo::new(conn);
        let category_result = category_repository.insert_one(&new_category)?;
        match create_creation_admin_log(
            category_result.to_string(),
            claims.user_id(),
            String::from("Blog Post Category"),
            String::from("categories"),
            &new_category_copy,
            String::from("/categories"),
            conn,
        ) {
            Ok(_) => {}
            Err(_) => {
                return Err(get_roll_back_err());
            }
        };
        Ok(simple_created_response(category_result))
    })
    .await?)
}
