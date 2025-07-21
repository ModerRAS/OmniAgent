use async_trait::async_trait;
use reqwest;
use serde::{Deserialize, Serialize};

use crate::llm::providers::{
    LLMError, LLMProvider, LLMRequest, LLMResponse, MessageRole as OurMessageRole, Usage,
};

#[derive(Debug, Clone)]
pub struct ClaudeProvider {
    pub api_key: String,
    pub model: String,
    pub base_url: String,
}

impl ClaudeProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            api_key,
            model: model.unwrap_or_else(|| "claude-3-sonnet-20240229".to_string()),
            base_url: "https://api.anthropic.com/v1".to_string(),
        }
    }

    fn convert_message_role(role: OurMessageRole) -> String {
        match role {
            OurMessageRole::System => "system".to_string(),
            OurMessageRole::User => "user".to_string(),
            OurMessageRole::Assistant => "assistant".to_string(),
        }
    }

    fn convert_messages(
        &self,
        messages: Vec<crate::llm::providers::Message>,
    ) -> (Option<String>, Vec<ClaudeMessage>) {
        let mut system_message = None;
        let mut claude_messages = Vec::new();

        for msg in messages {
            match msg.role {
                OurMessageRole::System => {
                    system_message = Some(msg.content);
                }
                _ => {
                    claude_messages.push(ClaudeMessage {
                        role: ClaudeProvider::convert_message_role(msg.role),
                        content: vec![ClaudeContent {
                            type_: "text".to_string(),
                            text: Some(msg.content),
                            source: None,
                        }],
                    });
                }
            }
        }

        (system_message, claude_messages)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ClaudeMessage {
    role: String,
    content: Vec<ClaudeContent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClaudeContent {
    #[serde(rename = "type")]
    type_: String,
    text: Option<String>,
    source: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClaudeRequestBody {
    model: String,
    max_tokens: u32,
    messages: Vec<ClaudeMessage>,
    system: Option<String>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    top_k: Option<u32>,
    stop_sequences: Option<Vec<String>>,
    stream: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct ClaudeResponseBody {
    #[allow(dead_code)]
    id: String,
    content: Vec<ClaudeResponseContent>,
    model: String,
    #[allow(dead_code)]
    role: String,
    #[allow(dead_code)]
    stop_reason: Option<String>,
    #[allow(dead_code)]
    stop_sequence: Option<String>,
    usage: ClaudeUsage,
}

#[derive(Debug, Deserialize)]
struct ClaudeResponseContent {
    #[serde(rename = "type")]
    #[allow(dead_code)]
    type_: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeUsage {
    input_tokens: u32,
    output_tokens: u32,
}

#[async_trait]
impl LLMProvider for ClaudeProvider {
    async fn chat(&self, request: LLMRequest) -> Result<LLMResponse, LLMError> {
        let client = reqwest::Client::new();

        let (system_message, claude_messages) = self.convert_messages(request.messages);

        let request_body = ClaudeRequestBody {
            model: self.model.clone(),
            max_tokens: request.max_tokens.unwrap_or(1024),
            messages: claude_messages,
            system: system_message,
            temperature: request.temperature,
            top_p: None,
            top_k: None,
            stop_sequences: None,
            stream: Some(false),
        };

        let response = client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| LLMError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(LLMError::ApiError(format!(
                "Claude API error: {error_text}"
            )));
        }

        let response_body: ClaudeResponseBody = response
            .json()
            .await
            .map_err(|e| LLMError::ApiError(format!("Parse error: {e}")))?;

        let content = response_body
            .content
            .first()
            .map(|c| c.text.clone())
            .unwrap_or_default();

        Ok(LLMResponse {
            content,
            model: response_body.model,
            usage: Some(Usage {
                prompt_tokens: response_body.usage.input_tokens,
                completion_tokens: response_body.usage.output_tokens,
                total_tokens: response_body.usage.input_tokens + response_body.usage.output_tokens,
            }),
        })
    }

    async fn chat_stream(&self, _request: LLMRequest) -> Result<String, LLMError> {
        Err(LLMError::ApiError(
            "Claude streaming not implemented yet".to_string(),
        ))
    }

    fn provider_name(&self) -> &'static str {
        "claude"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::providers::{Message, MessageRole};

    #[test]
    fn test_claude_provider_creation() {
        let provider = ClaudeProvider::new(
            "test-key".to_string(),
            Some("claude-3-opus-20240229".to_string()),
        );

        assert_eq!(provider.provider_name(), "claude");
        assert_eq!(provider.model, "claude-3-opus-20240229");
        assert_eq!(provider.base_url, "https://api.anthropic.com/v1");
    }

    #[test]
    fn test_convert_messages() {
        let provider = ClaudeProvider::new("test-key".to_string(), None);

        let messages = vec![
            Message {
                role: MessageRole::System,
                content: "You are a helpful assistant".to_string(),
            },
            Message {
                role: MessageRole::User,
                content: "Hello, Claude!".to_string(),
            },
        ];

        let (system_msg, claude_msgs) = provider.convert_messages(messages);

        assert_eq!(system_msg, Some("You are a helpful assistant".to_string()));
        assert_eq!(claude_msgs.len(), 1);
        assert_eq!(claude_msgs[0].role, "user");
        assert_eq!(
            claude_msgs[0].content[0].text,
            Some("Hello, Claude!".to_string())
        );
    }

    #[tokio::test]
    async fn test_claude_integration() {
        // This test requires ANTHROPIC_API_KEY environment variable
        if std::env::var("ANTHROPIC_API_KEY").is_err() {
            println!("Skipping Claude integration test - ANTHROPIC_API_KEY not set");
            return;
        }

        let api_key = std::env::var("ANTHROPIC_API_KEY").unwrap();
        let provider = ClaudeProvider::new(api_key, Some("claude-3-haiku-20240307".to_string()));

        let request = LLMRequest {
            messages: vec![Message {
                role: MessageRole::User,
                content: "Say hello in a very brief manner".to_string(),
            }],
            model: "claude-3-haiku-20240307".to_string(),
            temperature: Some(0.7),
            max_tokens: Some(50),
            stream: Some(false),
        };

        let result = provider.chat(request).await;

        match result {
            Ok(response) => {
                assert!(!response.content.is_empty());
                assert!(response.content.to_lowercase().contains("hello"));
                println!("Claude response: {}", response.content);
            }
            Err(e) => {
                panic!("Claude API test failed: {e}");
            }
        }
    }
}
