
CREATE TABLE admin_logs (
  id SERIAL PRIMARY KEY,
  change_message VARCHAR NOT NULL,
  object_id VARCHAR NOT NULL,
  user_id INTEGER NOT NULL,
  label VARCHAR NOT NULL,
  model VARCHAR NOT NULL,
  action_flag INTEGER NOT NULL,
  action_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT admin_log_user_fk
    FOREIGN KEY(user_id) 
	REFERENCES users(id)
);

CREATE UNIQUE INDEX idx_admin_logs_id 
ON admin_logs(id);

CREATE INDEX idx_admin_logs_user_id
ON admin_logs(user_id);

CREATE INDEX idx_admin_logs_action_flag
ON admin_logs(action_flag);

CREATE INDEX idx_admin_logs_model
ON admin_logs(action_flag);