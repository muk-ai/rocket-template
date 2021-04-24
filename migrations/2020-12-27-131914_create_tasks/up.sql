-- Your SQL goes here
CREATE TABLE tasks (
  id SERIAL PRIMARY KEY NOT NULL,
  description text NOT NULL,
  completed boolean NOT NULL DEFAULT false
);
