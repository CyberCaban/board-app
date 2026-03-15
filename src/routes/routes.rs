#[path = "routes/api_centrifugo_connect_route.rs"]
mod api_centrifugo_connect_route;
#[path = "routes/api_get_self_route.rs"]
mod api_get_self_route;
#[path = "routes/api_get_user_route.rs"]
mod api_get_user_route;
#[path = "routes/api_get_users_route.rs"]
mod api_get_users_route;
#[path = "routes/toro_route.rs"]
mod toro_route;

pub use api_centrifugo_connect_route::api_centrifugo_connect;
pub use api_get_self_route::api_get_self;
pub use api_get_user_route::api_get_user;
pub use api_get_users_route::api_get_users;
pub use toro_route::toro;
