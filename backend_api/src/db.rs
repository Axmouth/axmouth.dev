use backend_repo_pg::{
    blog_comments::BlogPostCommentRepo, blog_posts::BlogPostRepo, categories::CategoryRepo,
    change_password_tokens::ChangePasswordTokenRepo, home_page_links::HomePageLinkRepo,
    identification_cookies::IdentificationCookieRepo, projects::ProjectRepo,
    refresh_tokens::RefreshTokenRepo, technologies::TechnologyRepo, text_bodies::TextBodyRepo,
    uploaded_images::UploadedImageRepo, users::UserRepo, verify_email_tokens::VerifyEmailTokenRepo,
};
use backend_repo_pg::{page_views::PageViewRepo, pg_util::get_pg_pool};

/// A database "repository", for running database workloads.
#[derive(Clone)]
pub struct Repo {
    pub blog_post_repository: BlogPostRepo,
    pub blog_comment_repository: BlogPostCommentRepo,
    pub user_repository: UserRepo,
    pub refresh_token_repository: RefreshTokenRepo,
    pub project_repository: ProjectRepo,
    pub link_repository: HomePageLinkRepo,
    pub category_repository: CategoryRepo,
    pub technology_repository: TechnologyRepo,
    pub text_body_repository: TextBodyRepo,
    pub uploaded_images_repository: UploadedImageRepo,
    pub pages_views_repository: PageViewRepo,
    pub identification_cookies_repository: IdentificationCookieRepo,
    pub verify_email_tokens_repository: VerifyEmailTokenRepo,
    pub change_password_tokens_repository: ChangePasswordTokenRepo,
}

impl Repo {
    /// Creates a repo using default configuration for the underlying connection pool.
    pub async fn new(database_url: String) -> Self {
        let pool = get_pg_pool(database_url, 64);
        Self {
            blog_post_repository: BlogPostRepo::new(pool.clone()),
            blog_comment_repository: BlogPostCommentRepo::new(pool.clone()),
            user_repository: UserRepo::new(pool.clone()),
            refresh_token_repository: RefreshTokenRepo::new(pool.clone()),
            project_repository: ProjectRepo::new(pool.clone()),
            link_repository: HomePageLinkRepo::new(pool.clone()),
            category_repository: CategoryRepo::new(pool.clone()),
            technology_repository: TechnologyRepo::new(pool.clone()),
            text_body_repository: TextBodyRepo::new(pool.clone()),
            uploaded_images_repository: UploadedImageRepo::new(pool.clone()),
            pages_views_repository: PageViewRepo::new(pool.clone()),
            identification_cookies_repository: IdentificationCookieRepo::new(pool.clone()),
            verify_email_tokens_repository: VerifyEmailTokenRepo::new(pool.clone()),
            change_password_tokens_repository: ChangePasswordTokenRepo::new(pool.clone()),
        }
    }
}
