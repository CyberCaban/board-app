-- This file should undo anything in `up.sql`

DROP INDEX friends_idx;
ALTER TABLE users DROP COLUMN friends;
DROP TABLE friends_requests;
-- DROP INDEX friends_requests_idx;