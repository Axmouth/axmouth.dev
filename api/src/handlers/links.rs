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
use backend_repo_pg::home_page_links::HomePageLinkRepo;
use backend_repo_pg::models::queries::PaginatedQuery;
use backend_repo_pg::pg_util::get_roll_back_err;
use backend_repo_pg::pg_util::pg_transaction;
use backend_repo_pg::pg_util::DynRepo;
use backend_repo_pg::{
    change_sets::UpdateHomePageLink,
    filters::GetAllHomePageLinksFilter,
    insertables::NewHomePageLink,
    models::{
        queries::GetAllHomePageLinksQuery,
        requests::{CreateHomePageLinkRequest, UpdateHomePageLinkRequest},
    },
};
use tokio::task::block_in_place;

pub async fn get(
    Path(id): Path<i32>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    block_in_place(|| {
        let conn = repo.get_conn()?;
        let link_repository = HomePageLinkRepo::new(&conn);
        let link_result = match link_repository
            .find_one(id)
            .map_err::<PgRepoError, _>(|e| e.into())?
        {
            None => {
                return Ok(not_found_response("HomePageLink"));
            }
            Some(value) => value,
        };
        Ok(simple_ok_response(link_result))
    })
}

pub async fn get_all(
    query: GetAllHomePageLinksQuery,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    let filter = GetAllHomePageLinksFilter::from_query(query.clone());
    block_in_place(|| {
        let conn = repo.get_conn()?;
        let link_repository = HomePageLinkRepo::new(&conn);
        let pagination_opts = query.pagination_options();
        let sort_type = query.sort_type;
        let (links_list, total_results) = link_repository
            .find(filter, sort_type, pagination_opts)
            .map_err::<PgRepoError, _>(|e| e.into())?;
        Ok(paginated_ok_response(
            links_list,
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
        let link_repository = HomePageLinkRepo::new(conn);
        let old_data = match link_repository.find_one(id)? {
            None => {
                return Ok(not_found_response("HomePageLink"));
            }
            Some(value) => value,
        };
        let link_result = link_repository.delete_one(id)?;
        if link_result == 0 {
            return Ok(not_found_response("HomePageLink"));
        }
        match create_deletion_admin_log(
            id.to_string(),
            claims.user_id(),
            String::from("Link"),
            String::from("home_page_links"),
            &old_data,
            String::from("/links"),
            conn,
        ) {
            Ok(_) => {}
            Err(_) => {
                return Err(get_roll_back_err());
            }
        };
        Ok(simple_no_content_response(link_result))
    })
    .await?)
}

pub async fn update(
    Path(id): Path<i32>,
    AdminClaimsContext { claims }: AdminClaimsContext,
    ValidatedJson(request): ValidatedJson<UpdateHomePageLinkRequest>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    Ok(pg_transaction(repo, |conn| {
        let link_repository = HomePageLinkRepo::new(conn);
        let request_copy = request.clone();
        let old_data = match link_repository.find_one(id)? {
            Some(value) => value,
            None => {
                return Ok(not_found_response("Link"));
            }
        };
        let updated_link = UpdateHomePageLink {
            name: request.name,
            image: request.image,
            target: request.target,
        };
        let link_result = link_repository.update_one(id, &updated_link)?;
        match create_update_admin_log(
            id.to_string(),
            claims.user_id(),
            String::from("Link"),
            String::from("home_page_links"),
            &request_copy,
            &old_data,
            String::from("/links"),
            conn,
        ) {
            Ok(_) => {}
            Err(_) => {
                return Err(get_roll_back_err());
            }
        };
        Ok(simple_created_response(link_result))
    })
    .await?)
}

pub async fn create(
    AdminClaimsContext { claims }: AdminClaimsContext,
    ValidatedJson(request): ValidatedJson<CreateHomePageLinkRequest>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    let new_link = NewHomePageLink {
        name: request.name,
        image: request.image,
        target: request.target,
    };
    let new_link_copy = new_link.clone();
    Ok(pg_transaction(repo, |conn| {
        let link_repository = HomePageLinkRepo::new(conn);
        let link_result = link_repository.insert_one(new_link)?;
        match create_creation_admin_log(
            link_result.to_string(),
            claims.user_id(),
            String::from("Link"),
            String::from("home_page_links"),
            &new_link_copy,
            String::from("/links"),
            conn,
        ) {
            Ok(_) => {}
            Err(_) => {
                return Err(get_roll_back_err());
            }
        };
        Ok(simple_created_response(link_result))
    })
    .await?)
}
