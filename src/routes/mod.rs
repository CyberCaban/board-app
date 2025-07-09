mod auth_routes;
mod board_routes;
mod file_routes;
mod friend_routes;
mod mount_routes;
mod routes;
mod users_interaction;
pub trait AuthorizationRoutes {
    fn mount_auth_routes(self) -> Self;
    fn mount_board_routes(self) -> Self;
    fn manage_state(self) -> Self;
    async fn manage_rmq_stream(self) -> Self;
    fn mount_uploads(self) -> Self;
    fn mount_metrics(self) -> Self;
}
