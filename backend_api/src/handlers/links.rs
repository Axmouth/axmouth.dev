use crate::app::AppState;
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
use backend_repo_pg::errors::PgRepoError;
use backend_repo_pg::pg_util::{get_roll_back_err, RepoConnection};
use backend_repo_pg::{
    change_sets::UpdateHomePageLink,
    filters::GetAllHomePageLinksFilter,
    insertables::NewHomePageLink,
    models::{
        queries::GetAllHomePageLinksQuery,
        requests::{CreateHomePageLinkRequest, UpdateHomePageLinkRequest},
    },
};
use backend_repo_pg::{home_page_links::HomePageLinkRepo, options::PaginationOptions};
use tokio::task::block_in_place;

pub async fn get(id: i32, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    block_in_place(|| {
        let conn = RepoConnection::new(state.repo)?;
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
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let filter = GetAllHomePageLinksFilter::from_query(query.clone());
    block_in_place(|| {
        let conn = RepoConnection::new(state.repo)?;
        let link_repository = HomePageLinkRepo::new(&conn);
        let (links_list, total_results) = link_repository
            .find(
                filter,
                query.sort_type,
                PaginationOptions {
                    page: query.page,
                    page_size: query.page_size,
                },
            )
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
    id: i32,
    claims: Claims,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let link_repository = HomePageLinkRepo::new(&conn);
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
                state.repo.clone(),
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
    id: i32,
    claims: Claims,
    request: UpdateHomePageLinkRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let link_repository = HomePageLinkRepo::new(&conn);
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
                state.repo.clone(),
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
    claims: Claims,
    request: CreateHomePageLinkRequest,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let new_link = NewHomePageLink {
        name: request.name,
        image: request.image,
        target: request.target,
    };
    let new_link_copy = new_link.clone();
    Ok(state
        .repo
        .transaction(|conn| {
            let link_repository = HomePageLinkRepo::new(&conn);
            let link_result = link_repository.insert_one(new_link)?;
            match create_creation_admin_log(
                link_result.to_string(),
                claims.user_id(),
                String::from("Link"),
                String::from("home_page_links"),
                &new_link_copy,
                String::from("/links"),
                state.repo.clone(),
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
