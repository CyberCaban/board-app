use std::sync::Arc;

use diesel::{prelude::*, BoolExpressionMethods, ExpressionMethods};
use rocket::{
    tokio::{self, sync::broadcast},
    State,
};
use uuid::Uuid;
use ws::{Message, *};

use crate::{
    database::Db,
    errors::ApiError,
    jwt::Token,
    models::{
        api_response::ApiResponse,
        auth::AuthResult,
        messages::{ChatMessageDTO, ClientMessage},
        ws_state::{WsMessage, WsState},
    },
    schema::chat_messages,
};

async fn get_last_messages(
    db: Db,
    auth: Uuid,
) -> Result<ApiResponse<Vec<ChatMessageDTO>>, ApiResponse<ApiError>> {
    db.run(move |conn| {
        chat_messages::table
            .filter(
                chat_messages::receiver_id
                    .eq(auth)
                    .or(chat_messages::sender_id.eq(auth)),
            )
            .order_by(chat_messages::created_at.desc())
            .limit(100)
            .load::<ChatMessageDTO>(conn)
    })
    .await
    .map_err(|e| ApiResponse::from_error(e.into()))
    .map(|messages| ApiResponse::new(messages))
}

#[get("/last_messages")]
pub async fn last_messages(
    db: Db,
    auth: AuthResult,
) -> Result<ApiResponse<Vec<ChatMessageDTO>>, ApiResponse<ApiError>> {
    let auth = auth.unpack()?.id;
    get_last_messages(db, auth).await
}

#[get("/events")]
pub async fn events(ws: WebSocket, ws_state: &State<Arc<WsState>>, db: Db) -> Channel<'static> {
    use rocket::futures::{SinkExt, StreamExt};

    let ws_state = Arc::clone(ws_state);
    let (tx, rx) = broadcast::channel::<ChatMessageDTO>(16);
    let db = Arc::new(db);

    // Spawn DB task
    let db_clone = Arc::clone(&db);
    tokio::spawn(async move {
        let mut rx = rx;
        while let Ok(message) = rx.recv().await {
            let db = Arc::clone(&db_clone);
            tokio::spawn(async move {
                dbg!(&message);
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
            let (sink, mut receiver) = stream.split();
            let mut user_id: Option<Uuid> = None;

            // Wait for initial handshake
            while let Some(Ok(message)) = receiver.next().await {
                if let Message::Text(text) = message {
                    let user_data = match Token::decode_token(text) {
                        Ok(token) => token.claims.user,
                        Err(_) => return Ok(()),
                    };
                    user_id = Some(user_data.id);
                    ws_state.register(&user_data.id, sink).await;
                    break;
                }
            }

            // Handle remaining messages
            while let Some(Ok(message)) = receiver.next().await {
                match message {
                    Message::Text(text) => {
                        let message: ClientMessage =
                            serde_json::from_str(&text).unwrap_or_default();
                        let receiver_id = Uuid::parse_str(&message.receiver_id).unwrap_or_default();

                        let _ = ws_state
                            .send(&receiver_id, WsMessage::Chat(message.clone().into()))
                            .await;
                        if let Some(user_id) = user_id {
                            let _ = tx.send(message.clone().into());
                            let _ = ws_state
                                .send(&user_id, WsMessage::Chat(message.into()))
                                .await;
                        }
                    }

                    Message::Close(_) => {
                        if let Some(user_id) = user_id {
                            let _ = ws_state.unregister(&user_id).await;
                        }
                        break;
                    }
                    _ => {}
                }
            }

            Ok(())
        })
    })
}
