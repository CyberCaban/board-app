-- Your SQL goes here
ALTER TABLE users
ADD profile_url TEXT DEFAULT NULL, -- TODO: switch type to TEXT
ADD bio VARCHAR(255) DEFAULT NULL;
