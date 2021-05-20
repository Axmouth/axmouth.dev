use crate::app::AppState;
use crate::{
    auth_tokens,
    util::{
        not_found_response, paginated_ok_response, server_error_response, simple_created_response,
        simple_no_content_response, simple_ok_response,
    },
};
use auth_tokens::Claims;
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
    id: i32,
    claims: Option<Claims>,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let project_repository = ProjectRepo::new(state.repo.clone());
    let project_result = match project_repository.find_one(id).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value_opt) => match value_opt {
            None => {
                return Ok(not_found_response("Project"));
            }
            Some(value) => value,
        },
    };
    if let Some(claims) = claims {
        if claims.is_admin() == false && project_result.published == false {
            return Ok(not_found_response("Post"));
        }
    }
    Ok(simple_ok_response(project_result))
}

pub async fn get_all(
    query: GetAllProjectsQuery,
    claims: Option<Claims>,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut filter = GetAllProjectsFilter::from_query(query.clone());
    if let Some(claims) = claims {
        if claims.is_staff() == false {
            filter.published = Some(true);
        }
    } else {
        filter.published = Some(true);
    }
    let project_repository = ProjectRepo::new(state.repo.clone());
    let (projects_list, total_results) = match project_repository
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
        projects_list,
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
    let project_repository = ProjectRepo::new(state.repo.clone());
    let _ = match project_repository.find_one(id).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value_opt) => match value_opt {
            None => {
                return Ok(not_found_response("Project"));
            }
            Some(value) => value,
        },
    };
    let project_result = match project_repository.delete_one(id).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    if project_result == 0 {
        return Ok(not_found_response("Project"));
    }
    Ok(simple_no_content_response(project_result))
}

pub async fn update(
    id: i32,
    _claims: Claims,
    request: UpdateProjectRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let project_repository = ProjectRepo::new(state.repo.clone());
    let _ = match project_repository.find_one(id).await {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value_opt) => match value_opt {
            Some(value) => value,
            None => {
                return Ok(not_found_response("Project"));
            }
        },
    };
    let updated_project = UpdateProject {
        body: request.body,
        updated_at: Some(Some(Utc::now().naive_utc())),
        cover_image: request.cover_image,
        description: request.description,
        name: request.name,
        published: request.published,
    };
    if let Some(technologies_list) = request.technologies {
        let project_result = match project_repository
            .update_one_with_technologies(id, updated_project, technologies_list)
            .await
        {
            Err(err) => {
                return Ok(server_error_response(err));
            }
            Ok(value) => value,
        };
        Ok(simple_created_response(project_result))
    } else {
        let project_result = match project_repository.update_one(id, updated_project).await {
            Err(err) => {
                return Ok(server_error_response(err));
            }
            Ok(value) => value,
        };
        Ok(simple_created_response(project_result))
    }
}

pub async fn create(
    _claims: Claims,
    request: CreateProjectRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let new_project = NewProject {
        body: request.body,
        cover_image: request.cover_image,
        name: request.name,
        description: request.description,
    };
    let project_repository = ProjectRepo::new(state.repo.clone());
    let project_result = match project_repository
        .insert_one_with_technologies(new_project, request.technologies)
        .await
    {
        Err(err) => {
            return Ok(server_error_response(err));
        }
        Ok(value) => value,
    };
    Ok(simple_created_response(project_result))
}
