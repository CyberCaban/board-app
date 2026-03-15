use rocket::http::CookieJar;
use serde_json::{json, Value};

#[post("/logout")]
pub fn api_logout(jar: &CookieJar<'_>) -> Value {
    jar.remove("token");
    json!("Logged out")
}
