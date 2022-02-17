CREATE TABLE blog_post_comment_flags (
  id SERIAL PRIMARY KEY,
  reason VARCHAR NOT NULL,
  user_id INTEGER NOT NULL,
  blog_post_comment_id INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT blog_post_comment_flag_user_fk
    FOREIGN KEY(user_id) 
	REFERENCES users(id),
  CONSTRAINT blog_post_comment_flag_comment_fk
    FOREIGN KEY(blog_post_comment_id) 
	REFERENCES blog_post_comments(id)
);

CREATE UNIQUE INDEX idx_blog_post_comment_flags_id 
ON blog_post_comment_flags(id);

CREATE INDEX idx_blog_post_comment_flags_comment_id
ON blog_post_comment_flags(blog_post_comment_id);

CREATE INDEX idx_blog_post_comment_flags_user_id
ON blog_post_comment_flags(user_id);