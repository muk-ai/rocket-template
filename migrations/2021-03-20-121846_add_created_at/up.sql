-- Your SQL goes here
ALTER TABLE users ADD COLUMN created_at timestamptz NOT NULL DEFAULT current_timestamp;
ALTER TABLE tasks ADD COLUMN created_at timestamptz NOT NULL DEFAULT current_timestamp;
