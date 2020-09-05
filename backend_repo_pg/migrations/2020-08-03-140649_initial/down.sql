DROP INDEX idx_blog_posts_id;
DROP INDEX idx_blog_post_comments_id;
DROP INDEX idx_blog_post_comments_author_id;
DROP INDEX idx_blog_post_comments_post_id;
DROP INDEX idx_blog_posts_author_id;
DROP INDEX idx_projects_id;
DROP INDEX idx_refresh_tokens_id;
DROP INDEX idx_users_id;
DROP INDEX idx_users_email;

DROP TABLE projects;
DROP TABLE refresh_tokens;
DROP TABLE blog_post_comments;
DROP TABLE blog_posts;

DROP TABLE users;
DROP TYPE user_role;

DROP EXTENSION "uuid-ossp";