-- Your SQL goes here

CREATE TABLE IF NOT EXISTS conversations (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    member_one UUID NOT NULL REFERENCES users(id),
    member_two UUID NOT NULL REFERENCES users(id),
    UNIQUE (member_one, member_two)
);

CREATE INDEX ON conversations (member_one);
CREATE INDEX ON conversations (member_two);

CREATE TABLE IF NOT EXISTS chat_messages (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    sender_id UUID NOT NULL REFERENCES users(id),
    conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    file_id UUID REFERENCES files(id),
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX ON chat_messages (sender_id);
CREATE INDEX ON chat_messages (conversation_id);
