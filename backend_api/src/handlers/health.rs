use tokio::time::Instant;

use backend_repo_pg::{health::HealthRepo, models::responses::HealthResponse};

use crate::{
    app::AppState,
    util::{server_error_response, simple_ok_response},
};

pub async fn health(state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    let start = Instant::now();
    let api_up = true;
    let repo_check = state
        .repo
        .transaction(|conn| {
            let health_repo = HealthRepo::new(&conn);
            health_repo.check()
        })
        .await;
    let db_up = repo_check.is_ok();
    let db_response_time = repo_check.ok().flatten().map(|t| t.to_string());
    let api_response_time = Some(start.elapsed().as_millis().to_string());
    Ok(simple_ok_response(HealthResponse {
        api_up,
        api_response_time,
        db_up,
        db_response_time,
    }))
}
