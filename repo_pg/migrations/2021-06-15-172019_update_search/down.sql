DROP INDEX search_items_search_vec_idx;

DROP MATERIALIZED VIEW search_items;

CREATE TYPE search_item_type_new AS ENUM('Project', 'Blog Post', 'Page');

ALTER TABLE static_pages 
  ALTER COLUMN item_type TYPE search_item_type_new 
    USING (item_type::text::search_item_type_new);

CREATE MATERIALIZED VIEW search_items AS
  SELECT 
    id::VARCHAR || 'project'::VARCHAR as id,
    to_tsvector('english', name || ' ' || COALESCE(description, '') || ' ' || body) as search_vec,
    name as title,
    COALESCE(description, '') as description,
    'Project'::search_item_type_new as item_type,
    '/projects/' || slug::VARCHAR as link
    FROM projects
  UNION
  SELECT 
    id::VARCHAR || 'blogpost'::VARCHAR as id,
    to_tsvector('english', title || ' ' || COALESCE(description, '') || ' ' || body) as search_vec,
    title as title,
    COALESCE(description, '') as description,
    'Blog Post'::search_item_type_new as item_type,
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

DROP TYPE search_item_type;

ALTER TYPE search_item_type_new RENAME TO search_item_type;

CREATE INDEX search_items_search_vec_idx ON search_items USING GIN (search_vec);

