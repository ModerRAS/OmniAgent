use async_trait::async_trait;
use reqwest::Client;
use openai_api_rs::v1::chat_completion::{ChatCompletionRequest, ChatCompletionMessage, MessageRole, Content};
use openai_api_rs::v1::common::GPT3_5_TURBO;

use crate::llm::providers::{LLMProvider, LLMRequest, LLMResponse, LLMError, MessageRole as OurMessageRole, Usage};

#[derive(Debug)]
pub struct OpenAIProvider {
    client: Client,
    model: String,
    base_url: Option<String>,
    api_key: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String, model: Option<String>, base_url: Option<String>) -> Self {
        let client = Client::new();

        Self {
            client,
            model: model.unwrap_or_else(|| GPT3_5_TURBO.to_string()),
            base_url,
            api_key,
        }
    }

    fn convert_message_role(role: OurMessageRole) -> MessageRole {
        match role {
            OurMessageRole::System => MessageRole::system,
            OurMessageRole::User => MessageRole::user,
            OurMessageRole::Assistant => MessageRole::assistant,
        }
    }

    fn convert_messages(messages: Vec<crate::llm::providers::Message>) -> Vec<ChatCompletionMessage> {
        messages
            .into_iter()
            .map(|msg| ChatCompletionMessage {
                role: Self::convert_message_role(msg.role),
                content: Content::Text(msg.content),
                name: None,
                tool_call_id: None,
                tool_calls: None,
            })
            .collect()
    }
}

#[async_trait]
impl LLMProvider for OpenAIProvider {
    async fn chat(&self,
        request: LLMRequest,
    ) -> Result<LLMResponse, LLMError> {
        let messages = Self::convert_messages(request.messages);
        
        let req = ChatCompletionRequest::new(
            self.model.clone(),
            messages,
        )
        .temperature(request.temperature.unwrap_or(0.7) as f64)
        .max_tokens(request.max_tokens.unwrap_or(1000) as i64);

        let url = self.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
        let request_url = format!("{}/chat/completions", url);

        let response = self.client
            .post(&request_url)
            .bearer_auth(&self.api_key)
            .json(&req)
            .send()
            .await
            .map_err(|e| LLMError::ApiError(e.to_string()))?;

        let response_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| LLMError::ApiError(e.to_string()))?;

        let choices = response_data["choices"]
            .as_array()
            .ok_or_else(|| LLMError::ApiError("No choices in response".to_string()))?;

        let choice = choices.first()
            .ok_or_else(|| LLMError::ApiError("No response from OpenAI".to_string()))?;

        let content = choice["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let usage = response_data["usage"].as_object().map(|u| Usage {
            prompt_tokens: u["prompt_tokens"].as_u64().unwrap_or(0) as u32,
            completion_tokens: u["completion_tokens"].as_u64().unwrap_or(0) as u32,
            total_tokens: u["total_tokens"].as_u64().unwrap_or(0) as u32,
        });

        let model = response_data["model"]
            .as_str()
            .unwrap_or(&self.model)
            .to_string();

        Ok(LLMResponse {
            content,
            usage,
            model,
        })
    }

    async fn chat_stream(
        &self,
        _request: LLMRequest,
    ) -> Result<String, LLMError> {
        Err(LLMError::ApiError("OpenAI streaming not implemented yet".to_string()))
    }

    fn provider_name(&self
    ) -> &'static str {
        "openai"
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_openai_provider_creation() {
        let provider = OpenAIProvider::new(
            "test-key".to_string(),
            Some("gpt-3.5-turbo".to_string()),
            None,
        );
        
        assert_eq!(provider.provider_name(), "openai");
        assert_eq!(provider.model, "gpt-3.5-turbo");
    }

    #[tokio::test]
    async fn test_message_conversion() {
        let messages = vec![
            crate::llm::providers::Message {
                role: OurMessageRole::System,
                content: "You are a helpful assistant".to_string(),
            },
            crate::llm::providers::Message {
                role: OurMessageRole::User,
                content: "Hello, world!".to_string(),
            },
        ];

        let converted = OpenAIProvider::convert_messages(messages);
        assert_eq!(converted.len(), 2);
        assert_eq!(converted[0].role, MessageRole::system);
        assert_eq!(converted[1].role, MessageRole::user);
    }
}