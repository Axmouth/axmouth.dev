#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;
extern crate validator;

pub mod admin_logs;
pub mod blog_comments;
pub mod blog_post_comment_flags;
pub mod blog_post_comment_ratings;
pub mod blog_posts;
pub mod categories;
pub mod change_password_tokens;
pub mod change_sets;
pub mod entity;
pub mod errors;
pub mod extra;
pub mod filters;
pub mod health;
pub mod home_page_links;
pub mod identification_cookies;
pub mod insertables;
pub mod models;
pub mod options;
pub mod page_views;
pub mod passwords;
pub mod pg_util;
pub mod projects;
pub mod refresh_tokens;
pub mod schema;
pub mod schema_extra;
pub mod search_items;
pub mod technologies;
pub mod text_bodies;
pub mod uploaded_images;
pub mod users;
pub mod verify_email_tokens;

pub mod exports {
    // we will use that a bit later
    pub use super::extra::AdminLogActionType as Admin_log_action;
    pub use super::extra::SearchItemTypeType as Search_item_type;
    pub use super::extra::UserRoleType as User_role;
    pub use diesel_full_text_search::TsVector as Tsvector;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
