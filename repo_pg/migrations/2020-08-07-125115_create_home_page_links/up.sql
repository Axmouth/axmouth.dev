CREATE TABLE home_page_links (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  target VARCHAR NOT NULL,
  image VARCHAR NOT NULL
);

CREATE UNIQUE INDEX idx_home_page_links_id 
ON home_page_links(id);