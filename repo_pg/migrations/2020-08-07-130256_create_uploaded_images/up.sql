CREATE TABLE uploaded_images (
  id SERIAL PRIMARY KEY,
  extension VARCHAR NOT NULL,
  width INTEGER,
  height INTEGER,
  used_where VARCHAR,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  user_id INTEGER NOT NULL,
  CONSTRAINT uploaded_image_user_fk
    FOREIGN KEY(user_id) 
	REFERENCES users(id)
);

CREATE UNIQUE INDEX idx_uploaded_images_id 
ON uploaded_images(id);

CREATE INDEX idx_uploaded_images_extension
ON uploaded_images(extension);

CREATE INDEX idx_uploaded_images_user_id
ON uploaded_images(user_id);