use backend_repo_pg::{
    errors::PgRepoError,
    extra::UserRole,
    insertables::NewUser,
    passwords,
    pg_util::{get_pg_pool, Repo, RepoConnection},
    users::UserRepo,
};
use std::env;

pub fn parse_create_super_user_args(_: Vec<String>) {}

pub async fn create_super_user(
    display_name: String,
    email: String,
    password: String,
) -> Result<(), PgRepoError> {
    dotenv::dotenv().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let repo = get_pg_pool(database_url, 1).await;
    let conn = repo.get_async_conn();
    let user_repository = UserRepo::new(&conn);

    let new_user = NewUser {
        email,
        display_name,
        password: passwords::hash(password.as_bytes()),
        role: UserRole::Admin,
    };

    let user_result = user_repository.insert_one(new_user).await;

    let _ = user_result.expect("Could not create user");

    Ok(())
}
