-- This file should undo anything in `up.sql`
ALTER TABLE users
DROP COLUMN profile_url,
DROP COLUMN bio;