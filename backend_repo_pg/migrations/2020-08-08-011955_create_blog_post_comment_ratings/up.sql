CREATE TABLE blog_post_comment_ratings (
  id SERIAL PRIMARY KEY,
  is_like BOOLEAN NOT NULL,
  user_id INTEGER NOT NULL,
  blog_post_comment_id INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT blog_post_comment_ratings_user_fk
    FOREIGN KEY(user_id) 
	REFERENCES users(id),
  CONSTRAINT blog_post_comment_ratings_comment_fk
    FOREIGN KEY(blog_post_comment_id) 
	REFERENCES blog_post_comments(id)
);

CREATE UNIQUE INDEX idx_blog_post_comment_ratings_id 
ON blog_post_comment_ratings(id);

CREATE INDEX idx_blog_post_comment_ratings_is_like
ON blog_post_comment_ratings(is_like);

CREATE INDEX idx_blog_post_comment_ratings_user_id
ON blog_post_comment_ratings(user_id);

CREATE INDEX idx_blog_post_comment_ratings_comment_id
ON blog_post_comment_ratings(blog_post_comment_id);