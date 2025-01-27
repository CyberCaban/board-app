-- Your SQL goes here

ALTER TABLE users ADD COLUMN friends uuid[] default '{}';
CREATE INDEX friends_idx ON users (friends);

CREATE TABLE friends_requests (
    id uuid primary key,
    sender_id uuid not null references users (id),
    receiver_id uuid not null references users (id),
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);

CREATE INDEX friends_requests_idx ON friends_requests (sender_id, receiver_id);