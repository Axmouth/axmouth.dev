CREATE TABLE page_views (
  id SERIAL PRIMARY KEY,
  page_url VARCHAR NOT NULL,
  user_agent VARCHAR NOT NULL,
  user_location VARCHAR NOT NULL,
  id_hash VARCHAR NOT NULL,
  registered BOOLEAN NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_page_views_id 
ON page_views(id);

CREATE INDEX idx_page_views_page_url
ON page_views(page_url);

CREATE INDEX idx_page_views_id_hash
ON page_views(id_hash);