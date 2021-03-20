-- This file should undo anything in `up.sql`
ALTER TABLE users DROP COLUMN IF EXISTS created_at;
ALTER TABLE tasks DROP COLUMN IF EXISTS created_at;
