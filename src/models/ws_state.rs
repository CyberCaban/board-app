use std::{
    collections::{HashMap, HashSet},
    fmt,
    sync::Arc,
};

use rocket::{futures::stream::SplitSink, tokio::sync::RwLock};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use ws::{stream::DuplexStream, Message};

use rocket::futures::SinkExt;

use super::messages::ChatMessageDTO;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsMessage {
    Chat(ChatMessageDTO),
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

type ConversationId = Uuid;
type MemberId = Uuid;

/// State for the WebSocket server
///
/// This state is used to store the connections and conversations
///
/// The connections are stored in a HashMap with the member id as the key
///
/// The conversations are stored in a HashMap with the conversation id as the key
/// First the member is registered to the connections, then the member is added to the conversation
/// When the member is unregistered, the member is removed from the conversation
/// When the member is removed from the conversation, the member is removed from the connections
///
/// ```
/// let ws_state = WsState::new();
///  // add member to connections
/// ws_state.register_member(&member_id, &member);
///  // add member to conversation
/// ws_state.add_to_conversation(&conversation_id, &member_id);
///  // send message to conversation
/// ws_state.send(&conversation_id, &message);
///  // remove member from conversation
/// ws_state.unregister(&member_id);
/// ```
///

#[derive(Debug)]
pub struct WsState {
    connections: RwLock<HashMap<MemberId, Connection>>,
    conversations: RwLock<HashMap<ConversationId, HashSet<MemberId>>>,
    user_conversations: RwLock<HashMap<MemberId, ConversationId>>,
}

impl WsState {
    pub fn new() -> Arc<Self> {
        let state = Arc::new(Self {
            connections: RwLock::new(HashMap::new()),
            conversations: RwLock::new(HashMap::new()),
            user_conversations: RwLock::new(HashMap::new()),
        });

        state
    }

    pub async fn register_member(
        &self,
        member_id: &Uuid,
        member: SplitSink<DuplexStream, Message>,
    ) {
        self.connections
            .write()
            .await
            .insert(member_id.clone(), Connection { sender: member });
    }

    pub async fn add_to_conversation(&self, conv_id: &Uuid, member_id: &Uuid) {
        let mut guard = self.conversations.write().await;
        if guard.contains_key(conv_id) {
            guard.get_mut(conv_id).unwrap().insert(member_id.clone());
        } else {
            guard.insert(conv_id.clone(), HashSet::from([member_id.clone()]));
        }
        let mut user_guard = self.user_conversations.write().await;
        user_guard.insert(member_id.clone(), conv_id.clone());
    }

    pub async fn user_in_conversation(&self, conv_id: &Uuid, member_id: &Uuid) -> bool {
        let guard = self.conversations.read().await;
        if let Some(members) = guard.get(conv_id) {
            members.contains(member_id)
        } else {
            false
        }
    }

    pub async fn unregister(&self, member_id: &Uuid) -> WsResult<()> {
        let mut connections_guard = self.connections.write().await;
        if let Some(mut conn) = connections_guard.remove(member_id) {
            let res = conn.sender.flush().await;
            let _ = conn.sender.close().await;
            dbg!("Closing connection: ", &res);
        }

        let conv_id = {
            let mut user_guard = self.user_conversations.write().await;
            user_guard.remove(member_id)
        };
        if let Some(conv_id) = conv_id {
            let mut guard = self.conversations.write().await;
            if let Some(members) = guard.get_mut(&conv_id) {
                members.remove(member_id);
                if members.is_empty() {
                    guard.remove(&conv_id);
                }
            }
        }
        dbg!(&self);
        Ok(())
    }

    pub async fn send(&self, conv_id: &Uuid, message: WsMessage) -> WsResult<()> {
        dbg!(&self.conversations.read().await);
        dbg!(&self.connections.read().await);
        println!(
            "sending message: {:?}\n to conversation: {}",
            message, conv_id
        );

        let members = {
            let guard = self.conversations.read().await;
            if let Some(members) = guard.get(conv_id) {
                members.clone()
            } else {
                return Ok(());
            }
        };
        let mut connections = self.connections.write().await;

        for member_id in members {
            if let Some(connection) = connections.get_mut(&member_id) {
                let message_str = message.to_string();
                if let Err(e) = connection.sender.send(Message::Text(message_str)).await {
                    eprintln!("Failed to send message to {}: {}", member_id, e);
                }
            }
        }

        Ok(())
    }
}
