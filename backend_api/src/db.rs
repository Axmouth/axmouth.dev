use backend_repo_pg::{
    blog_comments::BlogPostCommentRepo, blog_posts::BlogPostRepo, categories::CategoryRepo,
    change_password_tokens::ChangePasswordTokenRepo, home_page_links::HomePageLinkRepo,
    identification_cookies::IdentificationCookieRepo, projects::ProjectRepo,
    refresh_tokens::RefreshTokenRepo, technologies::TechnologyRepo, text_bodies::TextBodyRepo,
    uploaded_images::UploadedImageRepo, users::UserRepo, verify_email_tokens::VerifyEmailTokenRepo,
};
use backend_repo_pg::{page_views::PageViewRepo, pg_util::get_pg_pool};
