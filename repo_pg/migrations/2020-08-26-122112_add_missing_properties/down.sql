ALTER TABLE uploaded_images
  DROP COLUMN path;

ALTER TABLE blog_posts
  DROP COLUMN description;

ALTER TABLE projects
  DROP COLUMN description;

ALTER TABLE projects
  DROP COLUMN cover_image;

ALTER TABLE projects
  DROP COLUMN name;

ALTER TABLE projects
  DROP COLUMN published;