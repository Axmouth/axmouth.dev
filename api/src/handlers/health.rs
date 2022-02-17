use axum::{extract::Extension, response::IntoResponse};
use tokio::{task::block_in_place, time::Instant};

use backend_repo_pg::{
    errors::PgRepoError, health::HealthRepo, models::responses::HealthResponse, pg_util::DynRepo,
};

use crate::util::simple_ok_response;

pub async fn health(Extension(repo): Extension<DynRepo>) -> impl IntoResponse {
    let start = Instant::now();
    let api_up = true;
    let repo_check: Result<Option<u128>, PgRepoError> = block_in_place(|| {
        let conn = repo.get_conn()?;
        let health_repo = HealthRepo::new(&conn);
        Ok(health_repo.check()?)
    });
    let db_up = repo_check.is_ok();
    let db_response_time = repo_check.ok().flatten().map(|t| t.to_string());
    let api_response_time = Some(start.elapsed().as_millis().to_string());
    simple_ok_response(HealthResponse {
        api_up,
        api_response_time,
        db_up,
        db_response_time,
    })
}
