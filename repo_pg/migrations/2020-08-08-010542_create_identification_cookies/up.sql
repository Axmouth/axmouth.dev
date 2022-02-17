CREATE TABLE identification_cookies (
  id SERIAL PRIMARY KEY,
  token VARCHAR NOT NULL,
  id_hash VARCHAR NOT NULL,
  expires_at TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX idx_identification_cookies_id 
ON identification_cookies(id);

CREATE UNIQUE INDEX idx_identification_cookies_token
ON identification_cookies(token);

CREATE INDEX idx_identification_cookies_id_hash
ON identification_cookies(id_hash);