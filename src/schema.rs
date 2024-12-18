// @generated automatically by Diesel CLI.

diesel::table! {
    board_column (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        position -> Int4,
        board_id -> Uuid,
    }
}

diesel::table! {
    board_users_relation (board_id, user_id) {
        board_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::table! {
    boards (id) {
        id -> Uuid,
        creator_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    card_attachments (file_id, card_id) {
        file_id -> Uuid,
        card_id -> Uuid,
    }
}

diesel::table! {
    column_card (id, column_id) {
        id -> Uuid,
        column_id -> Uuid,
        description -> Nullable<Text>,
        position -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        cover_attachment -> Nullable<Varchar>,
    }
}

diesel::table! {
    files (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        user_id -> Uuid,
        private -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        profile_url -> Nullable<Varchar>,
        #[max_length = 255]
        bio -> Nullable<Varchar>,
    }
}

diesel::joinable!(board_column -> boards (board_id));
diesel::joinable!(board_users_relation -> boards (board_id));
diesel::joinable!(board_users_relation -> users (user_id));
diesel::joinable!(boards -> users (creator_id));
diesel::joinable!(card_attachments -> files (file_id));
diesel::joinable!(column_card -> board_column (column_id));
diesel::joinable!(files -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    board_column,
    board_users_relation,
    boards,
    card_attachments,
    column_card,
    files,
    users,
);
