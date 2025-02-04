use std::{collections::HashMap, sync::Arc};

use rocket::tokio::sync::{
    broadcast::{self, Receiver, Sender},
    RwLock,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::friends::ChatMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsMessage {
    Chat(ChatMessage),
    Close,
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

#[derive(Debug, Clone)]
struct Connection {
    sender: Sender<WsMessage>,
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

    pub async fn register(&self, user_id: Uuid) -> Sender<WsMessage> {
        let (tx, _) = broadcast::channel(16);
        self.connections
            .write()
            .await
            .insert(user_id, Connection { sender: tx.clone() });
        tx
    }

    pub async fn unregister(&self, user_id: &Uuid) -> WsResult<()> {
        self.connections.write().await.remove(user_id);
        Ok(())
    }
    
    pub async fn send(&self, user_id: &Uuid, message: WsMessage) -> WsResult<()> {
        if let Some(connection) = self.connections.read().await.get(user_id) {
            if let Err(e) = connection.sender.send(message) {
                dbg!(&e);
                return Err(WsError::SendError(e.to_string()));
            }
        }

        Ok(())

    }
}
