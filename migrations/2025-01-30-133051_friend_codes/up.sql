-- Your SQL goes here

-- Add to your existing users table
ALTER TABLE users ADD COLUMN friend_code VARCHAR(9) UNIQUE;
ALTER TABLE users ADD COLUMN friend_code_expires_at TIMESTAMP;
-- Create new tables
CREATE TABLE friends (
    user_id UUID REFERENCES users(id),
    friend_id UUID REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, friend_id)
);

CREATE INDEX ON friends (user_id);
CREATE INDEX ON friends (friend_id);
