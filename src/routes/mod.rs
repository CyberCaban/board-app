mod mount_routes;
mod routes;
mod file_routes;
mod board_routes;
mod users_interaction;
pub trait AuthorizationRoutes {
    fn mount_auth_routes(self) -> Self;
    fn mount_board_routes(self) -> Self;
    fn mount_static_files(self) -> Self;
    fn mount_uploads(self) -> Self;
}
