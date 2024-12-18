use rocket::{fs::FileServer, Build, Rocket};

use super::{file_routes, routes, AuthorizationRoutes, board_routes::*};

impl AuthorizationRoutes for Rocket<Build> {
    fn mount_auth_routes(self) -> Self {
        self.mount(
            "/api",
            routes![
                file_routes::api_upload_file,
                file_routes::api_delete_file,
                file_routes::api_get_files,
                routes::api_get_user,
                routes::api_register,
                routes::api_login,
                routes::api_logout,
                routes::api_update_user,
                routes::toro
            ],
        )
    }

    fn mount_board_routes(self) -> Self {
        self.mount("/boards", routes![
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
        ],)
    }

    fn mount_static_files(self) -> Self {
        self.mount("/", FileServer::from("www/dist"))
    }

    fn mount_uploads(self) -> Self {
        self.mount("/uploads", FileServer::from("tmp").rank(1))
    }
}
