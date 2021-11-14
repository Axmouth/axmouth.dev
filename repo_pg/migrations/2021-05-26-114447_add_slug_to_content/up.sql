ALTER TABLE projects
  ADD slug VARCHAR;

UPDATE projects 
    SET slug = REPLACE(name, ' ', '_') || '-' || (floor(random() * 10000 + 1)::int)::varchar;

ALTER TABLE projects
    ALTER COLUMN slug SET NOT NULL;

ALTER TABLE blog_posts
  ADD slug VARCHAR;

UPDATE blog_posts 
    SET slug = REPLACE(title, ' ', '_') || '-' || (floor(random() * 10000 + 1)::int)::varchar;

ALTER TABLE blog_posts
    ALTER COLUMN slug SET NOT NULL;

CREATE UNIQUE INDEX idx_projects_slug 
    ON projects(slug);

CREATE UNIQUE INDEX idx_blog_posts_slug 
    ON blog_posts(slug);