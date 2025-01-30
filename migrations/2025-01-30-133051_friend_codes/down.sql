-- This file should undo anything in `up.sql`
-- ALTER TABLE users ADD COLUMN friends uuid[] default '{}';
ALTER TABLE users DROP COLUMN friend_code;
ALTER TABLE users DROP COLUMN friend_code_expires_at;
DROP TABLE friends;
