-- Your SQL goes here
CREATE TABLE deleted_users (
  id SERIAL PRIMARY KEY,
  user_id uuid NOT NULL,
  user_firebase_uid VARCHAR(128) NOT NULL,
  user_deleted_at timestamptz NOT NULL DEFAULT current_timestamp,
  user_created_at timestamptz NOT NULL
);
