mod mount_routes;
mod routes;
mod file_routes;
mod board_routes;
mod users_interaction;
mod auth_routes;
mod friend_routes;
pub trait AuthorizationRoutes {
    fn mount_auth_routes(self) -> Self;
    fn mount_board_routes(self) -> Self;
    fn manage_state(self) -> Self;
    fn mount_uploads(self) -> Self;
}
