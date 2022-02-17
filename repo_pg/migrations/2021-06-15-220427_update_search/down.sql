DROP TRIGGER refresh_static_page_search ON static_pages;
DROP TRIGGER refresh_link_search ON home_page_links;
DROP TRIGGER refresh_project_search ON projects;
DROP TRIGGER refresh_post_search ON blog_posts;
DROP FUNCTION refresh_search;

DROP MATERIALIZED VIEW search_items;

CREATE MATERIALIZED VIEW search_items AS
  SELECT 
    projects.id::VARCHAR || 'project'::VARCHAR as id,
    to_tsvector('english', projects.name || ' ' || COALESCE(description, '') || ' ' || body || ' ' || array_to_string(array_agg("technologies"."name"), ' ')) as search_vec,
    projects.name as title,
    projects.created_at as created_at,
    projects.updated_at as updated_at,
    projects.cover_image as cover_image,
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
    NULL as cover_image,
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
      NULL as cover_image,
      description,
      item_type,
      link
      FROM static_pages
  ;

CREATE INDEX search_items_search_vec_idx ON search_items USING GIN (search_vec);