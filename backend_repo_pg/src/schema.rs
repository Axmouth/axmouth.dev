table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    admin_logs (id) {
        id -> Int4,
        change_message -> Varchar,
        object_id -> Varchar,
        user_id -> Int4,
        label -> Varchar,
        model -> Varchar,
        action_flag -> Int4,
        action_time -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    blog_post_comment_flags (id) {
        id -> Int4,
        reason -> Varchar,
        user_id -> Int4,
        blog_post_comment_id -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    blog_post_comment_ratings (id) {
        id -> Int4,
        is_like -> Bool,
        user_id -> Int4,
        blog_post_comment_id -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    blog_post_comments (id) {
        id -> Int4,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        author_id -> Int4,
        post_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    blog_posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        published -> Bool,
        author_id -> Int4,
        description -> Nullable<Varchar>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    blog_posts_categories (id) {
        id -> Int4,
        category_id -> Int4,
        blog_post_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    categories (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    home_page_links (id) {
        id -> Int4,
        name -> Varchar,
        target -> Varchar,
        image -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    identification_cookies (id) {
        id -> Int4,
        token -> Varchar,
        id_hash -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    page_views (id) {
        id -> Int4,
        page_url -> Varchar,
        user_agent -> Varchar,
        user_location -> Varchar,
        id_hash -> Varchar,
        registered -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    projects (id) {
        id -> Int4,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        description -> Nullable<Varchar>,
        cover_image -> Nullable<Varchar>,
        name -> Varchar,
        published -> Bool,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    projects_technologies (id) {
        id -> Int4,
        technology_id -> Int4,
        project_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    refresh_tokens (id) {
        id -> Uuid,
        jwt_id -> Uuid,
        user_id -> Int4,
        invalidated -> Bool,
        used -> Bool,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    technologies (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    text_bodies (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        slug -> Varchar,
        body -> Text,
        url_used -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    uploaded_images (id) {
        id -> Int4,
        extension -> Varchar,
        width -> Nullable<Int4>,
        height -> Nullable<Int4>,
        used_where -> Nullable<Varchar>,
        created_at -> Timestamp,
        user_id -> Int4,
        path -> Varchar,
        url -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    users (id) {
        id -> Int4,
        display_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        role -> User_role,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::exports::*;

    verify_email_tokens (id) {
        id -> Int4,
        token -> Varchar,
        user_id -> Int4,
        email -> Varchar,
        old_email -> Nullable<Varchar>,
        invalidated -> Bool,
        used -> Bool,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

joinable!(admin_logs -> users (user_id));
joinable!(blog_post_comment_flags -> blog_post_comments (blog_post_comment_id));
joinable!(blog_post_comment_flags -> users (user_id));
joinable!(blog_post_comment_ratings -> blog_post_comments (blog_post_comment_id));
joinable!(blog_post_comment_ratings -> users (user_id));
joinable!(blog_post_comments -> blog_posts (post_id));
joinable!(blog_post_comments -> users (author_id));
joinable!(blog_posts -> users (author_id));
joinable!(blog_posts_categories -> blog_posts (blog_post_id));
joinable!(blog_posts_categories -> categories (category_id));
joinable!(projects_technologies -> projects (project_id));
joinable!(projects_technologies -> technologies (technology_id));
joinable!(refresh_tokens -> users (user_id));
joinable!(uploaded_images -> users (user_id));
joinable!(verify_email_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(
    admin_logs,
    blog_post_comment_flags,
    blog_post_comment_ratings,
    blog_post_comments,
    blog_posts,
    blog_posts_categories,
    categories,
    home_page_links,
    identification_cookies,
    page_views,
    projects,
    projects_technologies,
    refresh_tokens,
    technologies,
    text_bodies,
    uploaded_images,
    users,
    verify_email_tokens,
);
