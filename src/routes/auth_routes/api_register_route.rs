use rocket::http::CookieJar;
use rocket::serde::json::Json;

use crate::{
    database::Db,
    models::{api_response::ApiResponse, user::{PubUser, SignupDTO, User}},
};

#[post("/register", format = "json", data = "<user>")]
pub async fn api_register(
    db: Db,
    user: Json<SignupDTO>,
    jar: &CookieJar<'_>,
) -> Result<ApiResponse<PubUser>, ApiResponse> {
    User::signup(user.into_inner(), db, jar).await
}
