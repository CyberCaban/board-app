-- This file should undo anything in `up.sql`
ALTER TABLE "column_card"
DROP COLUMN "name",
DROP COLUMN "cover_attachment";

DROP TABLE IF EXISTS "card_attachments";