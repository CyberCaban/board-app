use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::time::{Duration, OffsetDateTime};
use serde_json::{json, Value};

use uuid::Uuid;

use crate::database::Db;
use crate::models::api_response::ApiResponse;
use crate::models::auth::AuthResult;
use crate::models::user::{LoginDTO, PubUser, SignupDTO, User};
use crate::schema::users;
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateUser {
    username: String,
    old_password: String,
    new_password: String,
    profile_url: String,
    bio: String,
}
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
) -> Result<ApiResponse<Uuid>, ApiResponse> {
    let user_token = auth.unpack()?.id;
    let new_user = new_user.into_inner();

    match db
        .run(move |conn| {
            diesel::update(users::table)
                .filter(users::id.eq(user_token))
                .set((
                    users::username.eq(&new_user.username.trim()),
                    users::profile_url.eq(&new_user.profile_url.trim()),
                    users::bio.eq(&new_user.bio.trim()),
                ))
                .returning(users::id)
                .get_result::<Uuid>(conn)
        })
        .await
    {
        Ok(user_id) => {
            let cookie = Cookie::build(("token", user_id.to_string()))
                .expires(OffsetDateTime::now_utc().checked_add(Duration::days(1)));
            jar.add(cookie);
            Ok(ApiResponse::new(user_id))
        }
        Err(e) => Err(ApiResponse::from_error(e.into())),
    }
}

#[post("/logout")]
pub fn api_logout(jar: &CookieJar<'_>) -> Value {
    jar.remove("token");
    json!("Logged out")
}
