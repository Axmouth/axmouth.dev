

CREATE TABLE categories (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE blog_posts_categories (
  id SERIAL PRIMARY KEY,
  category_id INTEGER NOT NULL,
  blog_post_id INTEGER NOT NULL,
  CONSTRAINT category_blog_post_fk
    FOREIGN KEY(category_id) 
    REFERENCES categories(id),
  CONSTRAINT blog_post_category_fk
    FOREIGN KEY(blog_post_id) 
	REFERENCES blog_posts(id)
);

CREATE UNIQUE INDEX idx_categories_id 
ON categories(id);

CREATE UNIQUE INDEX idx_categories_name 
ON categories(name);

CREATE INDEX idx_blog_posts_categories_blog_post_id
ON blog_posts_categories(blog_post_id);

CREATE INDEX idx_blog_posts_categories_category_id
ON blog_posts_categories(category_id);