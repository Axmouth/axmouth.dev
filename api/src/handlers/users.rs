use crate::{
    errors::AppError,
    util::{not_found_response, simple_ok_response},
};
use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
};
use backend_repo_pg::{
    models::responses::GetUserProfileResponse, pg_util::DynRepo, users::UserRepo,
};

pub async fn get(
    Path(id): Path<i32>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    let conn = repo.get_async_conn();
    let user_repository = UserRepo::new(&conn);
    let user = match user_repository.find_one(id).await? {
        None => {
            return Ok(not_found_response("User"));
        }
        Some(value) => value,
    };
    let user_profile_response = GetUserProfileResponse {
        id: user.id,
        created_at: user.created_at,
        display_name: user.display_name,
        role: user.role.into(),
    };
    Ok(simple_ok_response(user_profile_response))
}
