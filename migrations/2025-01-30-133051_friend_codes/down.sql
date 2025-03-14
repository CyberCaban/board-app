-- This file should undo anything in `up.sql`
-- ALTER TABLE users ADD COLUMN friends uuid[] default '{}';
DROP TABLE friends;
