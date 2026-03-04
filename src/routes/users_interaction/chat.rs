use std::sync::Arc;

use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{futures::SinkExt, serde::json::Json, State};
use serde::Deserialize;
use uuid::Uuid;
use ws::{Message, *};

use crate::{
    database::Db,
    errors::{ApiError, ApiErrorType},
    jwt::Token,
    models::{
        api_response::ApiResponse,
        auth::AuthResult,
        messages::{ChatMessageDTO, ClientMessage, Conversation},
        user::User,
        ws_state::{WsMessage, WsState},
    },
    schema::{chat_messages, conversations, users},
};

use super::helpers::get_last_messages;

#[get("/last_messages/<conversation_id>")]
pub async fn last_messages(
    db: Db,
    conversation_id: String,
) -> Result<ApiResponse<Vec<ChatMessageDTO>>, ApiResponse<ApiError>> {
    let conversation_id = Uuid::parse_str(&conversation_id);
    if conversation_id.is_err() {
        return Err(ApiResponse::new(ApiError::from_type(
            ApiErrorType::FailedToParseUUID,
        )));
    }
    get_last_messages(db, conversation_id.unwrap()).await
}

#[post("/message", data = "<message>")]
pub async fn send_message(
    db: Db,
    auth: AuthResult,
    message: Json<ClientMessage>,
) -> Result<ApiResponse<ChatMessageDTO>, ApiResponse<ApiError>> {
    let sender_id = auth.unpack()?.id;
    let conversation_id = Uuid::parse_str(&message.conversation_id)
        .map_err(|_| ApiResponse::new(ApiError::from_type(ApiErrorType::FailedToParseUUID)))?;

    let content = message.content.trim().to_string();
    if content.is_empty() {
        return Err(ApiResponse::new(ApiError::from_type(
            ApiErrorType::EmptyFields,
        )));
    }

    let (conversation, sender) = db
        .run(move |conn: &mut diesel::PgConnection| {
            let conv = conversations::table
                .filter(conversations::id.eq(conversation_id))
                .first::<Conversation>(conn)?;

            let sender = users::table
                .filter(users::id.eq(sender_id))
                .first::<User>(conn)?;
            Ok::<_, diesel::result::Error>((conv, sender.username))
        })
        .await
        .map_err(|e| ApiResponse::from_error(e.into()))?;

    if sender_id != conversation.member_one && sender_id != conversation.member_two {
        return Err(ApiResponse::new(ApiError::from_type(
            ApiErrorType::Unauthorized,
        )));
    }

    let recipient_id = if sender_id == conversation.member_one {
        conversation.member_two
    } else {
        conversation.member_one
    };

    let timestamp = Utc::now().naive_utc();
    let new_message = ChatMessageDTO {
        id: Uuid::new_v4(),
        sender_id,
        conversation_id,
        content,
        file_id: message.file_id,
        deleted: false,
        created_at: timestamp,
        updated_at: timestamp,
    };

    match db
        .run(move |conn| {
            diesel::insert_into(chat_messages::table)
                .values::<ChatMessageDTO>(new_message)
                .get_result::<ChatMessageDTO>(conn)
        })
        .await
    {
        Ok(saved_message) => {
            if let Err(error) =
                publish_chat_update_to_recipient(recipient_id, &saved_message, Some(&sender)).await
            {
                eprintln!("Centrifugo publish failed: {:?}", error);
            }
            Ok(ApiResponse::new(saved_message))
        }
        Err(e) => Err(ApiResponse::from_error(e.into())),
    }
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

#[derive(Deserialize, Default, Debug)]
struct Handshake {
    token: String,
    conversation_id: String,
}

#[get("/events")]
pub async fn events(ws: WebSocket, ws_state: &State<Arc<WsState>>, db: Db) -> Channel<'static> {
    use rocket::futures::StreamExt;

    let ws_state = Arc::clone(ws_state);
    let db = Arc::new(db);

    ws.channel(move |stream| {
        Box::pin(async move {
            let (mut sender, mut receiver) = stream.split();
            let mut user_id = Uuid::nil();

            // Wait for initial handshake
            if let Some(Ok(message)) = receiver.next().await {
                if let Message::Text(text) = message {
                    let handshake: Handshake = serde_json::from_str(&text).unwrap_or_default();
                    let user_data = match Token::decode_token(handshake.token) {
                        Ok(token) => token.claims.user,
                        Err(_) => return Ok(()),
                    };
                    let conv_id = Uuid::parse_str(&handshake.conversation_id).unwrap_or_default();

                    user_id = user_data.id;
                    let _ = sender
                        .send(Message::Text(format!(
                            "{{\"message\": \"Connected to chat: {}\"}}",
                            conv_id
                        )))
                        .await;

                    ws_state.register_member(&user_data.id, sender).await;
                    ws_state.add_to_conversation(&conv_id, &user_data.id).await;
                    dbg!("Succsessful handshake", &ws_state);
                }
            }

            // Handle remaining messages
            while let Some(Ok(message)) = receiver.next().await {
                match message {
                    Message::Text(text) => {
                        dbg!(&text);
                        let message: ClientMessage =
                            serde_json::from_str(&text).unwrap_or_default();
                        let conv_id = Uuid::parse_str(&message.conversation_id).unwrap_or_default();
                        if message.content.is_empty() {
                            continue;
                        }

                        dbg!(&message);
                        if !ws_state.user_in_conversation(&conv_id, &user_id).await {
                            ws_state.add_to_conversation(&conv_id, &user_id).await;
                        }

                        let _ = ws_state
                            .send(&conv_id, WsMessage::Chat(message.clone().into()))
                            .await;
                        // send message to db
                        let db_clone = Arc::clone(&db);
                        let _ = db_clone
                            .run(move |conn| {
                                diesel::insert_into(chat_messages::table)
                                    .values::<ChatMessageDTO>(message.clone().into())
                                    .execute(conn)
                            })
                            .await;
                    }

                    Message::Close(_) => {
                        dbg!("closing connection");
                        let _ = ws_state.unregister(&user_id).await;
                        dbg!("closing connection stop");
                        break;
                    }

                    _ => {
                        dbg!("unknown message");
                        break;
                    }
                }
            }

            Ok(())
        })
    })
}
