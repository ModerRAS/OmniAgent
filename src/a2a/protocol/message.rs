use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2AMessage {
    pub id: Uuid,
    pub sender: String,
    pub recipient: String,
    pub content: A2AMessageContent,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum A2AMessageContent {
    Text {
        text: String,
    },
    Request {
        request_type: String,
        payload: serde_json::Value,
    },
    Response {
        response_type: String,
        payload: serde_json::Value,
    },
    Error {
        code: String,
        message: String,
    },
}

impl A2AMessage {
    pub fn new(
        sender: String,
        recipient: String,
        content: A2AMessageContent,
        metadata: Option<serde_json::Value>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender,
            recipient,
            content,
            timestamp: chrono::Utc::now(),
            metadata,
        }
    }
}