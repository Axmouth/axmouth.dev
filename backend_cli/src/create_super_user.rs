use backend_models::User;
use backend_repo::{
    errors::MongoRepoError,
    mongo_repo::{get_db, get_db_collection},
    users,
};
use backend_shared::passwords;
use chrono::Utc;
use std::env;
use users::UserRepo;

pub fn parse_create_super_user_args(args: Vec<String>) {}

pub async fn create_super_user(
    display_name: String,
    email: String,
    password: String,
) -> Result<(), PgRepoError> {
    dotenv::dotenv().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = get_db(database_url).await.unwrap();
    let user_repository = UserRepo::new(get_db_collection(db.clone(), "users").await.unwrap());

    let new_user = User {
        _id: None,
        created_at: Utc::now().timestamp(),
        email,
        display_name,
        password: passwords::hash(password.as_bytes()),
        role: backend_models::UserRole::Admin,
        updated_at: None,
    };

    let user_result = user_repository.insert_one(new_user).await;

    if user_result.is_err() {};
    let user_result = user_result.unwrap();

    return Ok(());
}
