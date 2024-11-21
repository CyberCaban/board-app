// @generated automatically by Diesel CLI.

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

diesel::joinable!(files -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    files,
    users,
);
