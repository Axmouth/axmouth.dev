CREATE TABLE text_bodies (
  id SERIAL PRIMARY KEY,
  title VARCHAR,
  slug VARCHAR NOT NULL,
  body TEXT NOT NULL,
  url_used VARCHAR,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP
);

CREATE UNIQUE INDEX idx_text_bodies_id 
ON text_bodies(id);

CREATE UNIQUE INDEX idx_text_bodies_slug
ON text_bodies(slug);