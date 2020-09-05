extern crate diesel;
extern crate dotenv;

embed_migrations!();

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

pub fn get_pg_pool(database_url: String) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::new(database_url.as_str());
    let connection_pool = r2d2::Builder::default()
        .max_size(64)
        .build(manager)
        .expect("Could not instantiate db pool");
    let conn = connection_pool
        .get()
        .expect("Could not get db pool connection");
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
        .expect("Could not run migrations");
    connection_pool
}
