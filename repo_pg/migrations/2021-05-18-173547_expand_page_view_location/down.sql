ALTER TABLE page_views
  ADD user_location VARCHAR NOT NULL DEFAULT '';

ALTER TABLE page_views
  DROP COLUMN latitude;

ALTER TABLE page_views
  DROP COLUMN longitude;

ALTER TABLE page_views
  DROP COLUMN country_code;

UPDATE page_views 
    SET user_agent = ''
    WHERE user_agent IS NULL;

ALTER TABLE page_views
    ALTER COLUMN user_agent SET NOT NULL;