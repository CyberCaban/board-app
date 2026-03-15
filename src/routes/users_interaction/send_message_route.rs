use crate::{
    database::{users_interaction_queries::UsersInteractionQueries, Db},
    errors::ApiError,
    models::{
        api_response::ApiResponse,
        auth::AuthResult,
        messages::{ChatMessageDTO, ClientMessage},
    },
};
use rocket::serde::json::Json;
use uuid::Uuid;

#[post("/message", data = "<message>")]
pub async fn send_message(
    db: Db,
    auth: AuthResult,
    message: Json<ClientMessage>,
) -> Result<ApiResponse<ChatMessageDTO>, ApiResponse> {
    let auth = auth.unpack()?;
    let result = UsersInteractionQueries::send_message(&db, auth, message.into_inner())
        .await
        .map_err(ApiResponse::from_error)?;

    if let Err(error) = publish_chat_update_to_recipient(
        result.recipient_id,
        &result.saved_message,
        Some(&result.sender_username),
    )
    .await
    {
        eprintln!("Centrifugo publish failed: {:?}", error);
    }

    Ok(ApiResponse::new(result.saved_message))
}

async fn publish_chat_update_to_recipient(
    recipient_id: Uuid,
    saved_message: &ChatMessageDTO,
    sender: Option<&String>,
) -> Result<(), ApiError> {
    let centrifugo_http_url = std::env::var("CENTRIFUGO_HTTP_URL")
        .unwrap_or_else(|_| "http://localhost:8000".to_string());
    let centrifugo_api_key =
        std::env::var("CENTRIFUGO_API_KEY").unwrap_or_else(|_| "api_key".to_string());

    let publish_url = format!("{}/api/publish", centrifugo_http_url.trim_end_matches('/'));
    let channel = format!("chat#{}", recipient_id);

    reqwest::Client::new()
        .post(publish_url)
        .header("X-API-Key", centrifugo_api_key)
        .json(&serde_json::json!({
            "channel": channel,
            "data": {
                "sender": sender,
                "message": saved_message
            },
        }))
        .send()
        .await
        .map_err(ApiError::from_error)?
        .error_for_status()
        .map_err(ApiError::from_error)?;

    Ok(())
}
