extern crate diesel;
extern crate dotenv;

embed_migrations!();

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
#[derive(Clone)]
pub struct Repo {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

pub fn get_pg_pool(database_url: String, max_size: u32) -> Repo {
    let manager = ConnectionManager::new(database_url.as_str());
    let pool = r2d2::Builder::default()
        .max_size(max_size)
        .build(manager)
        .expect("Could not instantiate db pool");
    let conn = pool.get().expect("Could not get db pool connection");
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
        .expect("Could not run migrations");
    Repo { pool }
}
