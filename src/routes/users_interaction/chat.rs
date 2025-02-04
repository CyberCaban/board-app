use rocket::{
    futures::{SinkExt, StreamExt},
    tokio::{self, select, sync::broadcast::Sender},
    State,
};
use serde_json::json;
use ws::{Message, *};

use crate::models::friends::ChatMessage;

#[get("/events")]
pub async fn events(ws: WebSocket, queue: &State<Sender<ChatMessage>>) -> Channel<'static> {
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
