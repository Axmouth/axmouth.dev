CREATE TABLE verify_email_tokens (
  id SERIAL PRIMARY KEY,
  token VARCHAR NOT NULL,
  user_id INTEGER NOT NULL,
  email VARCHAR NOT NULL,
  old_email VARCHAR,
  invalidated BOOLEAN NOT NULL DEFAULT 'f',
  used BOOLEAN NOT NULL DEFAULT 'f',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  expires_at TIMESTAMP NOT NULL,
  CONSTRAINT verify_email_token_user_fk
    FOREIGN KEY(user_id) 
	REFERENCES users(id)
);

CREATE UNIQUE INDEX idx_verify_email_tokens_id 
ON verify_email_tokens(id);

CREATE UNIQUE INDEX idx_verify_email_tokens_token
ON verify_email_tokens(token);

CREATE INDEX idx_verify_email_tokens_user_id
ON verify_email_tokens(user_id);