use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::time::{Duration, OffsetDateTime};
use serde_json::{json, Value};

use crate::database::user_queries::{UpdateUser, UserQueries};
use crate::database::Db;
use crate::jwt;
use crate::models::api_response::ApiResponse;
use crate::models::auth::AuthResult;
use crate::models::user::{LoginDTO, PubUser, SignupDTO, User};

#[post("/register", format = "json", data = "<user>")]
pub async fn api_register(
    db: Db,
    user: Json<SignupDTO>,
    jar: &CookieJar<'_>,
) -> Result<ApiResponse<PubUser>, ApiResponse> {
    User::signup(user.into_inner(), db, jar).await
}

#[post("/login", format = "json", data = "<user>")]
pub async fn api_login(
    db: Db,
    user: Json<LoginDTO>,
    jar: &CookieJar<'_>,
) -> Result<ApiResponse<PubUser>, ApiResponse> {
    let user = user.into_inner();
    User::login(user, db, jar).await
}

#[put("/user", format = "json", data = "<new_user>")]
pub async fn api_update_user(
    db: Db,
    new_user: Json<UpdateUser>,
    jar: &CookieJar<'_>,
    auth: AuthResult,
) -> Result<ApiResponse<PubUser>, ApiResponse> {
    let user_token = auth.unpack()?.id;
    let new_user = new_user.into_inner();

    match UserQueries::update_profile(&db, new_user, user_token).await {
        Ok(user) => {
            let user_id: PubUser = PubUser::from(user.clone());
            let cookie = Cookie::build(("token", jwt::Token::generate_token(user_id)))
                .expires(OffsetDateTime::now_utc().checked_add(Duration::days(1)));
            jar.add(cookie);
            Ok(ApiResponse::new(user.into()))
        }
        Err(e) => Err(ApiResponse::from_error(e.into())),
    }
}

#[post("/logout")]
pub fn api_logout(jar: &CookieJar<'_>) -> Value {
    jar.remove("token");
    json!("Logged out")
}
