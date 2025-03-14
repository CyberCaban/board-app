-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    profile_url TEXT DEFAULT NULL,
    bio VARCHAR(255) DEFAULT NULL,
    friend_code VARCHAR(9) UNIQUE,
    friend_code_expires_at TIMESTAMP
);