use rocket::http::CookieJar;
use rocket::serde::json::Json;

use crate::{
    database::Db,
    models::{api_response::ApiResponse, user::{LoginDTO, PubUser, User}},
};

#[post("/login", format = "json", data = "<user>")]
pub async fn api_login(
    db: Db,
    user: Json<LoginDTO>,
    jar: &CookieJar<'_>,
) -> Result<ApiResponse<PubUser>, ApiResponse> {
    User::login(user.into_inner(), db, jar).await
}
