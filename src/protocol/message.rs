use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub sender: String,
    pub recipient: String,
    pub content: MessageContent,
    pub timestamp: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageContent {
    Text {
        text: String,
    },
    ToolCall {
        tool: String,
        parameters: serde_json::Value,
    },
    ToolResult {
        tool: String,
        result: serde_json::Value,
        success: bool,
    },
    AgentRequest {
        request_type: String,
        payload: serde_json::Value,
    },
    AgentResponse {
        response_type: String,
        payload: serde_json::Value,
    },
    Error {
        code: String,
        message: String,
    },
}

impl Message {
    pub fn new(
        sender: String,
        recipient: String,
        content: MessageContent,
        metadata: Option<serde_json::Value>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender,
            recipient,
            content,
            timestamp: Utc::now(),
            metadata,
        }
    }

    pub fn is_tool_call(&self) -> bool {
        matches!(self.content, MessageContent::ToolCall { .. })
    }

    pub fn is_agent_request(&self) -> bool {
        matches!(self.content, MessageContent::AgentRequest { .. })
    }

    pub fn is_error(&self) -> bool {
        matches!(self.content, MessageContent::Error { .. })
    }
}