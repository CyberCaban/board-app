use serde_json::{json, Value};

#[catch(404)]
pub fn not_found(req: &rocket::Request) -> String {
    let uri = req.uri();
    format!("Not Found at {}", uri)
}

#[catch(422)]
pub fn unprocessable_entity() -> Value {
    json!({ "error_msg":"Failed to parse: Missing Fields" })
}
