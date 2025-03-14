use std::sync::Arc;

use diesel::RunQueryDsl;
use rocket::{
    futures::SinkExt,
    tokio::{self, sync::broadcast},
    State,
};
use serde::Deserialize;
use uuid::Uuid;
use ws::{Message, *};

use crate::{
    database::Db,
    errors::{ApiError, ApiErrorType},
    jwt::Token,
    models::{
        api_response::ApiResponse,
        messages::{ChatMessageDTO, ClientMessage},
        ws_state::{WsMessage, WsState},
    },
    schema::chat_messages,
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

#[derive(Deserialize, Default, Debug)]
struct Handshake {
    token: String,
    conversation_id: String,
}

#[get("/events")]
pub async fn events(ws: WebSocket, ws_state: &State<Arc<WsState>>, db: Db) -> Channel<'static> {
    use rocket::futures::StreamExt;

    let ws_state = Arc::clone(ws_state);
    let (tx, mut rx) = broadcast::channel::<ChatMessageDTO>(16);
    let db = Arc::new(db);

    // Spawn DB task
    let db_clone = Arc::clone(&db);
    tokio::spawn(async move {
        while let Ok(message) = rx.recv().await {
            let db = Arc::clone(&db_clone);
            tokio::spawn(async move {
                let _ = db
                    .run(|conn| {
                        diesel::insert_into(chat_messages::table)
                            .values(message)
                            .execute(conn)
                    })
                    .await;
            });
        }
    });

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
                }
            }

            // Handle remaining messages
            while let Some(Ok(message)) = receiver.next().await {
                match message {
                    Message::Text(text) => {
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
                        let _ = tx.send(message.clone().into());
                    }

                    Message::Close(_) => {
                        dbg!("closing connection");
                        let _ = ws_state.unregister(&user_id).await;
                        break;
                    }

                    _ => {
                        dbg!("unknown message");
                    }
                }
            }

            Ok(())
        })
    })
}
