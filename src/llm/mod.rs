use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::protocol::message::{Message, MessageContent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub mock: bool,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            model: "mock-llm".to_string(),
            temperature: 0.7,
            max_tokens: 1000,
            mock: true,
        }
    }
}

pub struct MockLLM {
    config: LLMConfig,
}

impl MockLLM {
    pub fn new(config: LLMConfig) -> Self {
        Self { config }
    }

    pub async fn process_message(&self,
        input: &str,
        context: &[Message],
    ) -> Result<Message, String> {
        if self.config.mock {
            self.mock_process(input, context).await
        } else {
            Err("Real LLM not implemented".to_string())
        }
    }

    async fn mock_process(&self,
        input: &str,
        _context: &[Message],
    ) -> Result<Message, String> {
        // Simple mock processing
        let response_text = if input.contains("weather") {
            "I'll check the weather for you."
        } else if input.contains("calculate") {
            "I'll perform the calculation."
        } else if input.contains("mcp") {
            "I'll use an MCP tool to help you."
        } else if input.contains("a2a") {
            "I'll coordinate with another agent."
        } else {
            "I received your message: '{}'"
        };

        let content = if input.contains("tool") {
            MessageContent::ToolCall {
                tool: "mock_tool".to_string(),
                parameters: json!({"query": input}),
            }
        } else if input.contains("agent") {
            MessageContent::AgentRequest {
                request_type: "process".to_string(),
                payload: json!({"input": input}),
            }
        } else {
            MessageContent::Text {
                text: response_text.replace("{}", input),
            }
        };

        Ok(Message::new(
            "llm".to_string(),
            "user".to_string(),
            content,
            Some(json!({"model": &self.config.model})),
        ))
    }
}