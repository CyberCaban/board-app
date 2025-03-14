-- Your SQL goes here

CREATE TABLE friends (
    user_id UUID REFERENCES users(id),
    friend_id UUID REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, friend_id)
);

CREATE INDEX ON friends (user_id);
CREATE INDEX ON friends (friend_id);
