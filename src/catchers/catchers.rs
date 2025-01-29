#[catch(404)]
pub fn not_found(req: &rocket::Request) -> String {
    let uri = req.uri();
    format!("Not Found at {}", uri)
}

#[catch(422)]
pub fn unprocessable_entity() -> String {
    "Failed to parse: Missing Fields".to_string()
}