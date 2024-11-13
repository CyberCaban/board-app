use crate::database::Connection as Conn;
use rocket::{fs::FileServer, Build, Rocket};

use super::{file_routes, routes, AuthorizationRoutes};

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
