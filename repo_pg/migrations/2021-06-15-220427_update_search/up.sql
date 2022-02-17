DROP MATERIALIZED VIEW search_items;

CREATE MATERIALIZED VIEW search_items AS
  SELECT 
    projects.id::VARCHAR || 'project'::VARCHAR as id,
    to_tsvector('english', projects.name || ' ' || COALESCE(description, '') || ' ' || body || ' ' || array_to_string(array_agg("technologies"."name"), ' ')) as search_vec,
    projects.name as title,
    projects.created_at as created_at,
    projects.updated_at as updated_at,
    projects.cover_image as image,
    COALESCE(description, '') as description,
    'Project'::search_item_type as item_type,
    '/projects/' || slug::VARCHAR as link
    FROM projects
    LEFT JOIN projects_technologies ON projects_technologies.project_id = projects.id
    LEFT JOIN technologies ON technologies.id = projects_technologies.technology_id  
    WHERE published =  TRUE
    GROUP BY projects.id
  UNION
  SELECT 
    blog_posts.id::VARCHAR || 'project'::VARCHAR as id,
    to_tsvector('english', blog_posts.title || ' ' || COALESCE(description, '') || ' ' || body || ' ' || array_to_string(array_agg("categories"."name"), ' ')) as search_vec,
    blog_posts.title as title,
    blog_posts.created_at as created_at,
    blog_posts.updated_at as updated_at,
    NULL as image,
    COALESCE(description, '') as description,
    'Project'::search_item_type as item_type,
    '/projects/' || slug::VARCHAR as link
    FROM blog_posts
    LEFT JOIN blog_posts_categories ON blog_posts_categories.blog_post_id = blog_posts.id
    LEFT JOIN categories ON categories.id = blog_posts_categories.category_id 
    WHERE published =  TRUE
    GROUP BY blog_posts.id
  UNION
    SELECT
      id,
      search_vec,
      title,
      NULL as created_at,
      NULL as updated_at,
      NULL as image,
      description,
      item_type,
      link
      FROM static_pages
  UNION
  SELECT 
    home_page_links.id::VARCHAR || 'hpl'::VARCHAR as id,
    to_tsvector('english', home_page_links.name || ' ' || home_page_links.target || ' ' || home_page_links.image) as search_vec,
    home_page_links.name as title,
    NULL as created_at,
    NULL as updated_at,
    home_page_links.image as image,
    home_page_links.name as description,
    'External Link'::search_item_type as item_type,
    home_page_links.target as link
    FROM home_page_links
  ;

CREATE INDEX search_items_search_vec_idx ON search_items USING GIN (search_vec);

CREATE OR REPLACE FUNCTION refresh_search()
  RETURNS TRIGGER LANGUAGE plpgsql
  AS $$
  BEGIN
  REFRESH MATERIALIZED VIEW search_items;
  RETURN NULL;
  END $$;

CREATE TRIGGER refresh_post_search
  AFTER INSERT OR UPDATE OR DELETE OR TRUNCATE
  ON blog_posts
  FOR EACH STATEMENT
  EXECUTE PROCEDURE refresh_search();

CREATE TRIGGER refresh_project_search
  AFTER INSERT OR UPDATE OR DELETE OR TRUNCATE
  ON projects
  FOR EACH STATEMENT
  EXECUTE PROCEDURE refresh_search();

CREATE TRIGGER refresh_link_search
  AFTER INSERT OR UPDATE OR DELETE OR TRUNCATE
  ON home_page_links
  FOR EACH STATEMENT
  EXECUTE PROCEDURE refresh_search();

CREATE TRIGGER refresh_static_page_search
  AFTER INSERT OR UPDATE OR DELETE OR TRUNCATE
  ON static_pages
  FOR EACH STATEMENT
  EXECUTE PROCEDURE refresh_search();