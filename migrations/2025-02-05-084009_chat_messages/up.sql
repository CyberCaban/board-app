-- Your SQL goes here
CREATE TABLE IF NOT EXISTS chat_messages (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    sender_id UUID NOT NULL REFERENCES users(id),
    receiver_id UUID NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    file_id UUID REFERENCES files(id),
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX ON chat_messages (sender_id);
CREATE INDEX ON chat_messages (receiver_id);
