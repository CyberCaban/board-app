ALTER TABLE board_users_relation
    DROP CONSTRAINT IF EXISTS board_users_relation_board_id_fkey,
    ADD CONSTRAINT board_users_relation_board_id_fkey
        FOREIGN KEY (board_id) REFERENCES boards(id) ON DELETE CASCADE;

ALTER TABLE board_column
    DROP CONSTRAINT IF EXISTS board_column_board_id_fkey,
    ADD CONSTRAINT board_column_board_id_fkey
        FOREIGN KEY (board_id) REFERENCES boards(id) ON DELETE CASCADE;

ALTER TABLE column_card
    DROP CONSTRAINT IF EXISTS column_card_column_id_fkey,
    ADD CONSTRAINT column_card_column_id_fkey
        FOREIGN KEY (column_id) REFERENCES board_column(id) ON DELETE CASCADE;

ALTER TABLE card_attachments
    DROP CONSTRAINT IF EXISTS card_attachments_file_id_fkey,
    DROP CONSTRAINT IF EXISTS card_attachments_card_id_fkey,
    ADD CONSTRAINT card_attachments_file_id_fkey
        FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    ADD CONSTRAINT card_attachments_card_id_fkey
        FOREIGN KEY (card_id) REFERENCES column_card(id) ON DELETE CASCADE;

ALTER TABLE chat_messages
    DROP CONSTRAINT IF EXISTS chat_messages_file_id_fkey,
    ADD CONSTRAINT chat_messages_file_id_fkey
        FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE SET NULL;