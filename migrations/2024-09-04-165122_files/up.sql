-- Your SQL goes here
CREATE TABLE IF NOT EXISTS files (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    user_id UUID NOT NULL,
    private boolean NOT NULL DEFAULT true,
    FOREIGN KEY (user_id) REFERENCES users(id)
)