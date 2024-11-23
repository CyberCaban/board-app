-- Your SQL goes here
ALTER TABLE "column_card"
ADD COLUMN "name" VARCHAR(255) NOT NULL DEFAULT '',
ADD COLUMN "cover_attachment" VARCHAR(255) DEFAULT NULL;

CREATE TABLE IF NOT EXISTS "card_attachments" (
    "file_id" UUID NOT NULL,
    "card_id" UUID NOT NULL,
    PRIMARY KEY ("file_id", "card_id"),
    FOREIGN KEY ("file_id") REFERENCES "files" ("id"),
    FOREIGN KEY ("card_id") REFERENCES "column_card" ("id")
)