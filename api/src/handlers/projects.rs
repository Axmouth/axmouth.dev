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
use backend_repo_pg::errors::PgRepoError;
use backend_repo_pg::models::queries::GetProjectQuery;
use backend_repo_pg::models::queries::PaginatedQuery;
use backend_repo_pg::pg_util::get_roll_back_err;
use backend_repo_pg::pg_util::pg_transaction;
use backend_repo_pg::pg_util::DynRepo;
use backend_repo_pg::projects::ProjectRepo;
use backend_repo_pg::{
    change_sets::UpdateProject,
    filters::GetAllProjectsFilter,
    insertables::NewProject,
    models::{
        queries::GetAllProjectsQuery,
        requests::{CreateProjectRequest, UpdateProjectRequest},
    },
};
use chrono::Utc;
use tokio::task::block_in_place;

pub async fn get(
    Path(id): Path<String>,
    GetProjectQuery { use_slug }: GetProjectQuery,
    OptClaimsContext { claims }: OptClaimsContext,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    block_in_place(|| {
        let conn = repo.get_conn()?;
        let project_repository = ProjectRepo::new(&conn);
        let project_result = if let Some(true) = use_slug {
            match project_repository
                .find_one_by_slug(id)
                .map_err::<PgRepoError, _>(|e| e.into())?
            {
                None => {
                    return Ok(not_found_response("Project"));
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
            match project_repository
                .find_one(id)
                .map_err::<PgRepoError, _>(|e| e.into())?
            {
                None => {
                    return Ok(not_found_response("Project"));
                }
                Some(value) => value,
            }
        };
        if let Some(claims) = claims {
            if !claims.is_staff() && !project_result.published {
                return Ok(not_found_response("Post"));
            }
        }
        Ok(simple_ok_response(project_result))
    })
}

pub async fn get_all(
    query: GetAllProjectsQuery,
    OptClaimsContext { claims }: OptClaimsContext,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    block_in_place(|| {
        let conn = repo.get_conn()?;
        let mut filter = GetAllProjectsFilter::from_query(query.clone());
        if let Some(claims) = claims {
            if !claims.is_staff() {
                filter.published = Some(true);
            }
        } else {
            filter.published = Some(true);
        }
        let project_repository = ProjectRepo::new(&conn);
        let pagination_opts = query.pagination_options();
        let sort_type = query.sort_type;
        let (projects_list, total_results) = project_repository
            .find(filter, sort_type, pagination_opts)
            .map_err::<PgRepoError, _>(|e| e.into())?;
        Ok(paginated_ok_response(
            projects_list,
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
        let project_repository = ProjectRepo::new(conn);
        let old_data = match project_repository.find_one(id)? {
            None => {
                return Ok(not_found_response("Project"));
            }
            Some(value) => value,
        };
        let project_result = project_repository.delete_one(id)?;
        if project_result == 0 {
            return Ok(not_found_response("Project"));
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
        Ok(simple_no_content_response(project_result))
    })
    .await?)
}

pub async fn update(
    Path(id): Path<i32>,
    AdminClaimsContext { claims }: AdminClaimsContext,
    ValidatedJson(request): ValidatedJson<UpdateProjectRequest>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    Ok(pg_transaction(repo, |conn| {
        let project_repository = ProjectRepo::new(conn);
        let request_copy = request.clone();
        let old_data = match project_repository.find_one(id)? {
            Some(value) => value,
            None => {
                return Ok(not_found_response("Project"));
            }
        };
        let updated_project = UpdateProject {
            body: request.body,
            updated_at: Some(Some(Utc::now().naive_utc())),
            cover_image: request.cover_image,
            description: request.description,
            name: request.name,
            published: request.published,
            slug: request.slug,
        };
        let project_result = if let Some(technologies_list) = request.technologies {
            project_repository.update_one_with_technologies(
                id,
                &updated_project,
                technologies_list,
            )?
        } else {
            project_repository.update_one(id, &updated_project)?
        };
        match create_update_admin_log(
            id.to_string(),
            claims.user_id(),
            String::from("Project"),
            String::from("projects"),
            &request_copy,
            &old_data,
            String::from("/projects"),
            conn,
        ) {
            Ok(_) => {}
            Err(_) => {
                return Err(get_roll_back_err());
            }
        };
        Ok(simple_created_response(project_result))
    })
    .await?)
}

pub async fn create(
    AdminClaimsContext { claims }: AdminClaimsContext,
    ValidatedJson(request): ValidatedJson<CreateProjectRequest>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    Ok(pg_transaction(repo, |conn| {
        let request_copy = request.clone();
        let new_project = NewProject {
            body: request.body,
            cover_image: request.cover_image,
            name: request.name,
            description: request.description,
            slug: request.slug,
        };
        let project_repository = ProjectRepo::new(conn);
        let project_result =
            project_repository.insert_one_with_technologies(&new_project, request.technologies)?;
        match create_creation_admin_log(
            project_result.id.to_string(),
            claims.user_id(),
            String::from("Project"),
            String::from("projects"),
            &request_copy,
            String::from("/projects"),
            conn,
        ) {
            Ok(_) => {}
            Err(_) => {
                return Err(get_roll_back_err());
            }
        };
        Ok(simple_created_response(project_result))
    })
    .await?)
}
