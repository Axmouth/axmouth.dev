use crate::{
    app::AppState,
    util::{not_found_response, server_error_response, simple_ok_response},
};
use backend_repo_pg::{models::responses::GetUserProfileResponse, users::UserRepo};

// pub type Response = std::result::Result<warp::reply::Json, warp::Rejection>;

pub async fn get(id: i32, state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(state
        .repo
        .transaction(|conn| {
            let user_repository = UserRepo::new(&conn);
            let user = match user_repository.find_one(id)? {
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
        .await?)
}
