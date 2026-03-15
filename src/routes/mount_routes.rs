use rocket::{fs::FileServer, Build, Rocket};

use super::{
    auth_routes, board_routes, file_routes, friend_routes, routes, users_interaction,
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
                file_routes::api_get_file,
                routes::api_get_self,
                routes::api_get_user,
                routes::api_get_users,
                routes::toro,
                routes::api_centrifugo_connect,
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
        .mount(
            "/chat_source",
            routes![
                users_interaction::last_messages,
                users_interaction::send_message,
                users_interaction::get_or_create_conversation,
            ],
        )
    }

    fn mount_board_routes(self) -> Self {
        self.mount(
            "/boards",
            routes![
                board_routes::boards_create_board_and_relation,
                board_routes::boards_get_boards,
                board_routes::boards_get_board,
                board_routes::boards_update_board,
                board_routes::boards_delete_board,
                board_routes::boards_create_column,
                board_routes::boards_get_columns,
                board_routes::boards_get_column,
                board_routes::boards_update_column,
                board_routes::boards_delete_column,
                board_routes::boards_create_card,
                board_routes::boards_get_cards,
                board_routes::boards_get_card,
                board_routes::boards_update_card,
                board_routes::boards_delete_card,
                board_routes::boards_reorder_cards,
                board_routes::boards_get_card_by_id,
                board_routes::boards_add_attachment_to_card,
                board_routes::boards_get_attachments_of_card,
                board_routes::boards_delete_attachment_of_card,
                board_routes::boards_add_collaborator,
                board_routes::boards_get_collaborators,
                board_routes::boards_get_collaborator,
                board_routes::boards_remove_collaborator,
            ],
        )
    }

    fn manage_state(self) -> Self {
        self
    }

    fn mount_uploads(self) -> Self {
        self.mount("/uploads", FileServer::from("tmp").rank(1))
    }

    fn mount_metrics(self) -> Self {
        let prom = rocket_prometheus::PrometheusMetrics::new();
        self.attach(prom.clone()).mount("/metrics", prom)
    }
}
