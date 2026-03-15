use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::time::{Duration, OffsetDateTime};

use crate::{
    database::Db,
    database::user_queries::{UpdateUser, UserQueries},
    jwt,
    models::{api_response::ApiResponse, auth::AuthResult, user::PubUser},
};

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
