use crate::{
    app::AppState,
    util::{not_found_response, simple_ok_response},
};
use backend_repo_pg::{
    errors::PgRepoError, models::responses::GetUserProfileResponse, pg_util::RepoConnection,
    users::UserRepo,
};
use tokio::task::block_in_place;

pub async fn get(id: i32, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    block_in_place(|| {
        let conn = RepoConnection::new(state.repo)?;
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
