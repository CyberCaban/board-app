use crate::database::PSQLConnection as Conn;
use rocket::{fs::FileServer, Build, Rocket};

use super::{file_routes, routes, AuthorizationRoutes, board_routes};

impl AuthorizationRoutes for Rocket<Build> {
    fn mount_auth_routes(self) -> Self {
        self.mount(
            "/api",
            routes![
                file_routes::api_upload_file,
                file_routes::api_get_file,
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
            board_routes::boards_add_collaborator,
            board_routes::boards_get_collaborators,
            board_routes::boards_get_collaborator,
            board_routes::boards_remove_collaborator
        ],)
    }

    fn mount_static_files(self) -> Self {
        self.mount("/", FileServer::from("www/dist"))
    }

    fn mount_uploads(self) -> Self {
        self.mount("/uploads", FileServer::from("tmp").rank(1))
    }

    fn manage_db(self) -> Self {
        self.manage(Conn::new())
    }
}
