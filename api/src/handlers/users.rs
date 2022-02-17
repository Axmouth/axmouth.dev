use crate::{
    errors::AppError,
    util::{not_found_response, simple_ok_response},
};
use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
};
use backend_repo_pg::{
    errors::PgRepoError, models::responses::GetUserProfileResponse, pg_util::DynRepo,
    users::UserRepo,
};
use tokio::task::block_in_place;

pub async fn get(
    Path(id): Path<i32>,
    Extension(repo): Extension<DynRepo>,
) -> Result<impl IntoResponse, AppError> {
    block_in_place(|| {
        let conn = repo.get_conn()?;
        let user_repository = UserRepo::new(&conn);
        let user = match user_repository
            .find_one(id)
            .map_err::<PgRepoError, _>(|e| e.into())?
        {
            None => {
                return Ok(not_found_response("User"));
            }
            Some(value) => value,
        };
        let user_profile_response = GetUserProfileResponse {
            id: user.id,
            created_at: user.created_at,
            display_name: user.display_name,
            role: user.role,
        };
        Ok(simple_ok_response(user_profile_response))
    })
}
