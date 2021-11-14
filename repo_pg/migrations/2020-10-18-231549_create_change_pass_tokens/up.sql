CREATE TABLE change_password_tokens (
  id SERIAL PRIMARY KEY,
  token VARCHAR NOT NULL,
  user_id INTEGER NOT NULL,
  invalidated BOOLEAN NOT NULL DEFAULT 'f',
  used BOOLEAN NOT NULL DEFAULT 'f',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  expires_at TIMESTAMP NOT NULL,
  CONSTRAINT change_password_token_user_fk
    FOREIGN KEY(user_id) 
	REFERENCES users(id)
);

CREATE UNIQUE INDEX idx_change_password_tokens_id 
ON change_password_tokens(id);

CREATE UNIQUE INDEX idx_change_password_tokens_token
ON change_password_tokens(token);

CREATE INDEX idx_change_password_tokens_user_id
ON change_password_tokens(user_id);