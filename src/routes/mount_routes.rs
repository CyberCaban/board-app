use rocket::{fs::FileServer, Build, Rocket};

use crate::models::ws_state::WsState;

use super::{
    auth_routes,
    board_routes::*,
    file_routes, friend_routes, routes,
    users_interaction::{self, chat::*, conversations::*},
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
            routes![events, last_messages, get_or_create_conversation],
        )
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

    fn manage_state(self) -> Self {
        let ws_state = WsState::new();
        self.manage(ws_state)
    }

    fn mount_uploads(self) -> Self {
        self.mount("/uploads", FileServer::from("tmp").rank(1))
    }

    fn mount_metrics(self) -> Self {
        let prom = rocket_prometheus::PrometheusMetrics::new();
        self.attach(prom.clone()).mount("/metrics", prom)
    }
}
