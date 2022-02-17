ALTER TABLE uploaded_images
  ADD path VARCHAR NOT NULL DEFAULT '';

ALTER TABLE blog_posts
  ADD description VARCHAR;

ALTER TABLE projects
  ADD description VARCHAR;

ALTER TABLE projects
  ADD cover_image VARCHAR;

ALTER TABLE projects
  ADD name VARCHAR NOT NULL;

ALTER TABLE projects
  ADD published BOOLEAN NOT NULL DEFAULT 'f';