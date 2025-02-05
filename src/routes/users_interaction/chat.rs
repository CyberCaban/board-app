use std::sync::Arc;

use rocket::{
    tokio::{self, select, sync::broadcast::{self, Receiver}},
    State,
};
use serde_json::json;
use uuid::Uuid;
use ws::{Message, *};

use crate::{jwt::Token, models::{
    friends::ChatMessage,
    ws_state::{WsMessage, WsState},
}};

#[get("/events")] // TODO: Rewrite to use ws::Stream
pub async fn events(ws: WebSocket, ws_state: &State<Arc<WsState>>) -> Channel<'static> {
    use rocket::futures::{SinkExt, StreamExt};
    
    let ws_state = Arc::clone(ws_state);
    
    ws.channel(move |stream| {
        Box::pin(async move {
            let (sink, mut receiver) = stream.split();
            
            // Wait for initial handshake
            while let Some(Ok(message)) = receiver.next().await {
                if let Message::Text(text) = message {
                    let user_data = match Token::decode_token(text) {
                        Ok(token) => token.claims.user,
                        Err(_) => return Ok(()),
                    };
                    ws_state.register(&user_data.id, sink).await;
                    break;
                }
            }

            // Handle remaining messages
            while let Some(Ok(message)) = receiver.next().await {
                match message {
                    Message::Text(text) => {
                        let m: ChatMessage = serde_json::from_str(&text).unwrap_or_default();
                        let receiver_id = Uuid::parse_str(&m.receiver_id).unwrap();
                        let _ = ws_state.send(&receiver_id, WsMessage::Chat(m)).await;
                    }
                    Message::Close(_) => break,
                    _ => {}
                }
            }

            Ok(())
        })
    })
}
