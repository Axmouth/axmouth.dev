use crate::{app::AppState, util::simple_ok_response};

pub async fn health(state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(simple_ok_response(()))
}
