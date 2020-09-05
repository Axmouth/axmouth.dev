CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE user_role as enum('Admin', 'Moderator', 'User', 'Ghost');

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  display_name VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP,
  role user_role NOT NULL
);

CREATE TABLE blog_posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP,
  published BOOLEAN NOT NULL DEFAULT 'f',
  author_id INTEGER NOT NULL,
  CONSTRAINT post_author_fk
    FOREIGN KEY(author_id) 
	  REFERENCES users(id)
);

CREATE TABLE blog_post_comments (
  id SERIAL PRIMARY KEY,
  body TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP,
  author_id INTEGER NOT NULL,
  post_id INTEGER NOT NULL,
  CONSTRAINT comment_post_fk
    FOREIGN KEY(post_id) 
	  REFERENCES blog_posts(id),
  CONSTRAINT comment_author_fk
    FOREIGN KEY(author_id) 
	  REFERENCES users(id)
);

CREATE TABLE refresh_tokens (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  jwt_id UUID NOT NULL,
  user_id INTEGER NOT NULL,
  invalidated BOOLEAN NOT NULL DEFAULT 'f',
  used BOOLEAN NOT NULL DEFAULT 'f',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  expires_at TIMESTAMP NOT NULL,
  CONSTRAINT refresh_token_user_fk
    FOREIGN KEY(user_id) 
	  REFERENCES users(id)
);

CREATE TABLE projects (
  id SERIAL PRIMARY KEY,
  body TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP
);

CREATE UNIQUE INDEX idx_blog_posts_id 
ON blog_posts(id);

CREATE UNIQUE INDEX idx_blog_post_comments_id 
ON blog_post_comments(id);

CREATE INDEX idx_blog_post_comments_author_id 
ON blog_post_comments(author_id);

CREATE INDEX idx_blog_post_comments_post_id 
ON blog_post_comments(post_id);

CREATE INDEX idx_blog_posts_author_id 
ON blog_posts(author_id);

CREATE UNIQUE INDEX idx_projects_id 
ON projects(id);

CREATE UNIQUE INDEX idx_refresh_tokens_id 
ON refresh_tokens(id);

CREATE UNIQUE INDEX idx_users_id 
ON users(id);

CREATE UNIQUE INDEX idx_users_email 
ON users(email);