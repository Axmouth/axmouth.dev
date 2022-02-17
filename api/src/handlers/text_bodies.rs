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
use backend_repo_pg::errors::PgRepoError;
use backend_repo_pg::models::queries::PaginatedQuery;
use backend_repo_pg::pg_util::get_roll_back_err;
use backend_repo_pg::pg_util::pg_transaction;
use backend_repo_pg::pg_util::DynRepo;
use backend_repo_pg::text_bodies::TextBodyRepo;
use backend_repo_pg::{
    change_sets::UpdateTextBody,
    filters::GetAllTextBodiesFilter,
    insertables::NewTextBody,
    models::{
        queries::GetAllTextBodiesQuery,
        requests::{CreateTextBodyRequest, UpdateTextBodyRequest},
    },
};
use chrono::Utc;
use tokio::task::block_in_place;

pub async fn get(
    Path(slug): Path<String>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    block_in_place(|| {
        let conn = repo.get_conn()?;
        let text_body_repository = TextBodyRepo::new(&conn);
        let text_body_result = match text_body_repository
            .find_one_by_slug(slug)
            .map_err::<PgRepoError, _>(|e| e.into())?
        {
            None => {
                return Ok(not_found_response("TextBody"));
            }
            Some(value) => value,
        };
        Ok(simple_ok_response(text_body_result))
    })
}

pub async fn get_all(
    query: GetAllTextBodiesQuery,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    block_in_place(|| {
        let conn = repo.get_conn()?;
        let filter = GetAllTextBodiesFilter::from_query(query.clone());
        let text_body_repository = TextBodyRepo::new(&conn);
        let pagination_opts = query.pagination_options();
        let sort_type = query.sort_type;
        let (text_bodies_list, total_results) = text_body_repository
            .find(filter, sort_type, pagination_opts)
            .map_err::<PgRepoError, _>(|e| e.into())?;
        Ok(paginated_ok_response(
            text_bodies_list,
            query.page,
            query.page_size,
            total_results,
        ))
    })
}

pub async fn delete(
    Path(slug): Path<String>,
    AdminClaimsContext { claims }: AdminClaimsContext,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    Ok(pg_transaction(repo, |conn| {
        let text_body_repository = TextBodyRepo::new(conn);
        let old_entity = match text_body_repository.find_one_by_slug(slug)? {
            None => {
                return Ok(not_found_response("TextBody"));
            }
            Some(value) => value,
        };
        let text_body_result = text_body_repository.delete_one(old_entity.id)?;
        if text_body_result == 0 {
            return Ok(not_found_response("TextBody"));
        }
        match create_deletion_admin_log(
            old_entity.id.to_string(),
            claims.user_id(),
            String::from("Text Body"),
            String::from("text_bodies"),
            &old_entity,
            String::from("/text-bodies"),
            conn,
        ) {
            Ok(_) => {}
            Err(_) => {
                return Err(get_roll_back_err());
            }
        };
        Ok(simple_no_content_response(text_body_result))
    })
    .await?)
}

pub async fn update(
    Path(slug): Path<String>,
    AdminClaimsContext { claims }: AdminClaimsContext,
    ValidatedJson(request): ValidatedJson<UpdateTextBodyRequest>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    Ok(pg_transaction(repo, |conn| {
        let text_body_repository = TextBodyRepo::new(conn);
        let request_copy = request.clone();
        let old_entity = match text_body_repository.find_one_by_slug(slug)? {
            Some(value) => value,
            None => {
                return Ok(not_found_response("Link"));
            }
        };
        let updated_text_body = UpdateTextBody {
            body: request.body,
            slug: request.slug.clone(),
            title: request.title,
            url_used: request.url_used,
            updated_at: Some(Utc::now().naive_utc()),
        };
        let text_body_result =
            text_body_repository.update_one(old_entity.id, &updated_text_body)?;
        match create_update_admin_log(
            old_entity.id.to_string(),
            claims.user_id(),
            String::from("Text Body"),
            String::from("text_bodies"),
            &request_copy,
            &old_entity,
            String::from("/text-bodies"),
            conn,
        ) {
            Ok(_) => {}
            Err(_) => {
                return Err(get_roll_back_err());
            }
        };
        Ok(simple_created_response(text_body_result))
    })
    .await?)
}

pub async fn create(
    AdminClaimsContext { claims }: AdminClaimsContext,
    ValidatedJson(request): ValidatedJson<CreateTextBodyRequest>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    Ok(pg_transaction(repo, |conn| {
        let new_text_body = NewTextBody {
            body: request.body,
            slug: request.slug,
            title: request.title,
            url_used: request.url_used,
        };
        let new_text_body_copy = new_text_body.clone();
        let text_body_repository = TextBodyRepo::new(conn);
        let text_body_result = text_body_repository.insert_one(&new_text_body)?;
        match create_creation_admin_log(
            text_body_result.id.to_string(),
            claims.user_id(),
            String::from("Text Body"),
            String::from("text_bodies"),
            &new_text_body_copy,
            String::from("/text-bodies"),
            conn,
        ) {
            Ok(_) => {}
            Err(_) => {
                return Err(get_roll_back_err());
            }
        };
        Ok(simple_created_response(text_body_result))
    })
    .await?)
}
