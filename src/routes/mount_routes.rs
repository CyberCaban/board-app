use rocket::{fs::FileServer, tokio::sync::broadcast::channel, Build, Rocket};

use crate::models::friends::ChatMessage;

use super::{
    auth_routes,
    board_routes::*,
    file_routes, friend_routes, routes,
    users_interaction::{self, chat::events},
    AuthorizationRoutes,
};

impl AuthorizationRoutes for Rocket<Build> {
    fn mount_auth_routes(self) -> Self {
        self.mount(
            "/api",
            routes![
                file_routes::api_upload_file,
                file_routes::api_delete_file,
                file_routes::api_get_files,
                routes::api_get_self,
                routes::api_get_user,
                routes::api_get_users,
                routes::toro,
                auth_routes::api_register,
                auth_routes::api_login,
                auth_routes::api_logout,
                auth_routes::api_update_user,
            ],
        )
        .mount(
            "/friends",
            routes![
                users_interaction::get_friends,
                friend_routes::generate_friend_code,
                friend_routes::get_friend_code,
                friend_routes::redeem_friend_code,
            ],
        )
        .manage(channel::<ChatMessage>(1024).0)
        .mount("/chat_source", routes![events])
    }

    fn mount_board_routes(self) -> Self {
        self.mount(
            "/boards",
            routes![
                base_actions::boards_create_board_and_relation,
                base_actions::boards_get_boards,
                base_actions::boards_get_board,
                base_actions::boards_update_board,
                base_actions::boards_delete_board,
                column_actions::boards_create_column,
                column_actions::boards_get_columns,
                column_actions::boards_get_column,
                column_actions::boards_update_column,
                column_actions::boards_delete_column,
                card_actions::boards_create_card,
                card_actions::boards_get_cards,
                card_actions::boards_get_card,
                card_actions::boards_update_card,
                card_actions::boards_delete_card,
                card_actions::boards_reorder_cards,
                card_editing::boards_get_card_by_id,
                card_editing::boards_add_attachment_to_card,
                card_editing::boards_get_attachments_of_card,
                card_editing::boards_delete_attachment_of_card,
                collaborator_actions::boards_add_collaborator,
                collaborator_actions::boards_get_collaborators,
                collaborator_actions::boards_get_collaborator,
                collaborator_actions::boards_remove_collaborator,
            ],
        )
    }

    fn mount_static_files(self) -> Self {
        self.mount("/", FileServer::from("www/dist"))
    }

    fn mount_uploads(self) -> Self {
        self.mount("/uploads", FileServer::from("tmp").rank(1))
    }
}
