ALTER TABLE page_views
  DROP COLUMN user_location;

ALTER TABLE page_views
  ADD latitude DOUBLE PRECISION;

ALTER TABLE page_views
  ADD longitude DOUBLE PRECISION;

ALTER TABLE page_views
  ADD country_code VARCHAR;

ALTER TABLE page_views
    ALTER COLUMN user_agent DROP NOT NULL;