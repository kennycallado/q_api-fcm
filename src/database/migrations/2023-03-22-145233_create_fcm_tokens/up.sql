CREATE TABLE IF NOT EXISTS fcm_tokens ();

ALTER TABLE fcm_tokens
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN user_id INTEGER UNIQUE NOT NULL,
  ADD COLUMN token VARCHAR(255),
  ADD COLUMN created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  ADD COLUMN updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP;

SELECT diesel_manage_updated_at('fcm_tokens');
INSERT INTO fcm_tokens (user_id, token) VALUES
  (1,  ''),
  (2,  null),
  (4,  null),
  (5,  null),
  (6,  null),
  (7,  null),
  (8,  null),
  (9,  null),
  (10, null)
  ;
  -- ON CONFLICT DO NOTHING;
