use std::sync::Arc;

use rocket::{
    tokio::{self, select},
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
    let mut is_handshake = true;
    ws.channel(move |stream| {
        Box::pin(async move {
            println!("Connected");
            let (mut sender, mut receiver) = stream.split();
            tokio::spawn(async move {
                loop {
                    select! {
                        Some(Ok(message)) = receiver.next() => {
                            match message {
                                ws::Message::Text(text) => {
                                    if is_handshake {
                                        let user_data = match Token::decode_token(text.to_string()) {
                                            Ok(token) => token.claims.user,
                                            Err(_) => {
                                                let _ = sender.close().await;
                                                break;
                                            }
                                        };
                                        dbg!(&user_data.username);

                                        is_handshake = false;
                                        continue;
                                    }
                                    // Handle Text message
                                    println!("Received Text message: {}", text);
                                    let m: ChatMessage = match serde_json::from_str(&text) {
                                        Ok(m) => m,
                                        Err(_) => ChatMessage::default(),
                                    };
                                    let res = m.clone();
                                    let _ = sender.send(Message::Text(json!(res).to_string())).await;
                                }

                                    ws::Message::Close(close_frame) => {
                                        // Handle Close message
                                        println!("Received Close message: {:?}", close_frame);
                                        let close_frame = ws::frame::CloseFrame {
                                            code: ws::frame::CloseCode::Normal,
                                            reason: "Client disconected".to_string().into(),
                                        };
                                        let _ = sender.close().await;
                                        break;
                                    }

                                    _ => {
                                        println!("Received other message: {:?}", message);
                                    }
                                }
                        }
                        else => {
                                println!("Connection closed");
                                let close_frame = ws::frame::CloseFrame {
                                    code: ws::frame::CloseCode::Normal,
                                    reason: "Client disconected".to_string().into(),
                                };
                                let _ = sender.close().await;
                                break;
                            }
                    }
                }
            });

            tokio::signal::ctrl_c().await.unwrap();
            Ok(())
        })
    })
}
