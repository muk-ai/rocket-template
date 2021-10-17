-- This file should undo anything in `up.sql`
ALTER TABLE users RENAME COLUMN firebase_uid to uid;
