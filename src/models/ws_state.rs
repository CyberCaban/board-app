use std::{collections::HashMap, fmt, sync::Arc};

use rocket::{
    futures::stream::SplitSink,
    tokio::sync::{
        broadcast::{self, Receiver, Sender},
        RwLock,
    },
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use ws::{stream::DuplexStream, Message};

use super::friends::ChatMessage;
use rocket::futures::{SinkExt, StreamExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsMessage {
    Chat(ChatMessage),
    Close,
}

impl std::fmt::Display for WsMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WsMessage::Chat(msg) => write!(f, "{}", serde_json::to_string(msg).unwrap()),
            WsMessage::Close => write!(f, "Close"),
        }
    }
}

type WsResult<T> = Result<T, WsError>;

#[derive(Debug)]
pub enum WsError {
    ConnectionError(String),
    SendError(String),
}

impl std::fmt::Display for WsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WsError::ConnectionError(e) => write!(f, "Connection error: {}", e),
            WsError::SendError(e) => write!(f, "Send error: {}", e),
        }
    }
}

struct Connection {
    sender: SplitSink<DuplexStream, Message>,
}

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Connection")
    }
}

pub struct WsState {
    pub connections: RwLock<HashMap<Uuid, Connection>>,
}

impl WsState {
    pub fn new() -> Arc<Self> {
        let state = Arc::new(Self {
            connections: RwLock::new(HashMap::new()),
        });

        state
    }

    pub async fn contains_key(&self, user_id: Uuid) -> bool {
        self.connections.read().await.contains_key(&user_id)
    }

    pub async fn register(&self, user_id: &Uuid, sender: SplitSink<DuplexStream, Message>) {
        self.connections
            .write()
            .await
            .insert(user_id.clone(), Connection { sender });
    }

    pub async fn unregister(&self, user_id: &Uuid) -> WsResult<()> {
        self.connections.write().await.remove(user_id);
        Ok(())
    }

    pub async fn send(&self, user_id: &Uuid, message: WsMessage) -> WsResult<()> {
        dbg!(&self.connections.read().await);
        if let Some(connection) = self.connections.write().await.get_mut(user_id) {
            connection
                .sender
                .send(Message::Text(message.to_string()))
                .await;
        }

        Ok(())
    }
}
