-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "boards" (
    "id" UUID UNIQUE DEFAULT uuid_generate_v4 (),
    "creator_id" UUID NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    PRIMARY KEY ("id"),
    FOREIGN KEY ("creator_id") REFERENCES "users" ("id")
);

CREATE TABLE IF NOT EXISTS "board_users_relation" (
    "board_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    PRIMARY KEY ("board_id", "user_id"),
    FOREIGN KEY ("board_id") REFERENCES "boards" ("id"),
    FOREIGN KEY ("user_id") REFERENCES "users" ("id")
);

CREATE TABLE IF NOT EXISTS "board_column" (
    "id" UUID UNIQUE DEFAULT uuid_generate_v4 (),
    "name" VARCHAR(255),
    "position" INTEGER NOT NULL,
    "board_id" UUID NOT NULL,
    PRIMARY KEY ("id"),
    FOREIGN KEY ("board_id") REFERENCES "boards" ("id")
);

CREATE TABLE IF NOT EXISTS "column_card" (
    "id" UUID UNIQUE DEFAULT uuid_generate_v4 (),
    "column_id" UUID NOT NULL,
    "description" TEXT,
    "position" INTEGER NOT NULL,
    PRIMARY KEY ("id", "column_id"),
    FOREIGN KEY ("column_id") REFERENCES "board_column" ("id")
)