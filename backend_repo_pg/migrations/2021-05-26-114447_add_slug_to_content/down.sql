DROP INDEX idx_blog_posts_slug;
DROP INDEX idx_projects_slug;

ALTER TABLE projects
  DROP COLUMN slug;

ALTER TABLE blog_posts
  DROP COLUMN slug;