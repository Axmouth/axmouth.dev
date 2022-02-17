CREATE TYPE search_item_type as enum('Project', 'Blog Post', 'Page');

CREATE TABLE static_pages(
    id VARCHAR PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    search_vec TSVECTOR NOT NULL,
    item_type search_item_type NOT NULL,
    link VARCHAR NOT NULL
);

INSERT INTO static_pages(id, title, description, search_vec, link, item_type)
    VALUES ('1pages', 'Contact Me', 'Send me an email to contact me',to_tsvector('english', 'Send me an email to contact me'), '/contact-me', 'Page');

INSERT INTO static_pages(id, title, description, search_vec, link, item_type)
    VALUES ('2pages', 'Login', 'Login to your account', to_tsvector('english', 'Login to your account'), '/auth/login', 'Page');

INSERT INTO static_pages(id, title, description, search_vec, link, item_type)
    VALUES ('3pages', 'Register', 'Create a new account to this website', to_tsvector('english', 'Register Create a new account to this website'), '/auth/register', 'Page');

INSERT INTO static_pages(id, title, description, search_vec, link, item_type)
    VALUES ('4pages', 'Account Settings', 'Settings for your website account', to_tsvector('english', 'Account Settings for your account'), '/auth/settings', 'Page');

CREATE MATERIALIZED VIEW search_items AS
  SELECT 
    id::VARCHAR || 'project'::VARCHAR as id,
    to_tsvector('english', name || ' ' || COALESCE(description, '') || ' ' || body) as search_vec,
    name as title,
    COALESCE(description, '') as description,
    'Project'::search_item_type as item_type,
    '/projects/' || slug::VARCHAR as link
    FROM projects
  UNION
  SELECT 
    id::VARCHAR || 'blogpost'::VARCHAR as id,
    to_tsvector('english', title || ' ' || COALESCE(description, '') || ' ' || body) as search_vec,
    title as title,
    COALESCE(description, '') as description,
    'Blog Post'::search_item_type as item_type,
    '/blog/' || slug::VARCHAR as link
    FROM blog_posts where published =  TRUE
  UNION
    SELECT
      id,
      search_vec,
      title,
      description,
      item_type,
      link
      FROM static_pages
  ;

CREATE INDEX search_items_search_vec_idx ON search_items USING GIN (search_vec);