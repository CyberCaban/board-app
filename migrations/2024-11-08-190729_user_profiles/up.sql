-- Your SQL goes here
ALTER TABLE users
ADD profile_url VARCHAR(255) DEFAULT NULL, -- TODO: make url type TEXT
ADD bio VARCHAR(255) DEFAULT NULL;
