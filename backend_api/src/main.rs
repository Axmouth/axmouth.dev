pub mod app;
pub mod db;
pub mod handlers;
// pub mod repository;
pub mod routes;
// pub mod schema;
pub mod auth_tokens;
pub mod cookies;
pub mod emails;
pub mod errors;
pub mod filters;
pub mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    env_logger::init();

    app::start().await;
    Ok(())
}
