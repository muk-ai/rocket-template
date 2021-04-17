-- Your SQL goes here
ALTER TABLE users ADD COLUMN id uuid NOT NULL UNIQUE DEFAULT uuid_generate_v4();
