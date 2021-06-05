use crate::app::AppState;
use crate::util::bad_request_response;
use crate::util::create_creation_admin_log;
use crate::util::create_deletion_admin_log;
use crate::util::create_update_admin_log;
use crate::{
    auth_tokens,
    util::{
        not_found_response, paginated_ok_response, simple_created_response,
        simple_no_content_response, simple_ok_response,
    },
};
use auth_tokens::Claims;
use backend_repo_pg::models::queries::GetProjectQuery;
use backend_repo_pg::pg_util::get_roll_back_err;
use backend_repo_pg::{
    change_sets::UpdateProject,
    filters::GetAllProjectsFilter,
    insertables::NewProject,
    models::{
        queries::GetAllProjectsQuery,
        requests::{CreateProjectRequest, UpdateProjectRequest},
    },
};
use backend_repo_pg::{options::PaginationOptions, projects::ProjectRepo};
use chrono::Utc;

pub async fn get(
    id: String,
    query: GetProjectQuery,
    claims: Option<Claims>,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let project_repository = ProjectRepo::new(&conn);
            let project_result = if let Some(true) = query.use_slug {
                match project_repository.find_one_by_slug(id)? {
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
                match project_repository.find_one(id)? {
                    None => {
                        return Ok(not_found_response("Project"));
                    }
                    Some(value) => value,
                }
            };
            if let Some(claims) = claims {
                if claims.is_staff() == false && project_result.published == false {
                    return Ok(not_found_response("Post"));
                }
            }
            Ok(simple_ok_response(project_result))
        })
        .await?)
}

pub async fn get_all(
    query: GetAllProjectsQuery,
    claims: Option<Claims>,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let mut filter = GetAllProjectsFilter::from_query(query.clone());
            if let Some(claims) = claims {
                if claims.is_staff() == false {
                    filter.published = Some(true);
                }
            } else {
                filter.published = Some(true);
            }
            let project_repository = ProjectRepo::new(&conn);
            let (projects_list, total_results) = project_repository.find(
                filter,
                query.sort_type,
                PaginationOptions {
                    page: query.page,
                    page_size: query.page_size,
                },
            )?;
            Ok(paginated_ok_response(
                projects_list,
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
            let project_repository = ProjectRepo::new(&conn);
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
                String::from("/api/v1/links"),
                state.repo.clone(),
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
    id: i32,
    claims: Claims,
    request: UpdateProjectRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let project_repository = ProjectRepo::new(&conn);
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
                String::from("/api/v1/projects"),
                state.repo.clone(),
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
    claims: Claims,
    request: CreateProjectRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let request_copy = request.clone();
            let new_project = NewProject {
                body: request.body,
                cover_image: request.cover_image,
                name: request.name,
                description: request.description,
                slug: request.slug,
            };
            let project_repository = ProjectRepo::new(&conn);
            let project_result = project_repository
                .insert_one_with_technologies(&new_project, request.technologies)?;
            match create_creation_admin_log(
                project_result.id.to_string(),
                claims.user_id(),
                String::from("Project"),
                String::from("projects"),
                &request_copy,
                String::from("/api/v1/projects"),
                state.repo.clone(),
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
