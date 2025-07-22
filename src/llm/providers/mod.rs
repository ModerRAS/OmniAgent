pub mod claude;
pub mod google;
pub mod openai;

pub use openai::OpenAIProvider;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LLMError {
    #[error("API request failed: {0}")]
    ApiError(String),
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    #[error("Rate limit exceeded: {0}")]
    RateLimitError(String),
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMRequest {
    pub messages: Vec<Message>,
    pub model: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    pub content: String,
    pub usage: Option<Usage>,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn chat(&self, request: LLMRequest) -> Result<LLMResponse, LLMError>;
    async fn chat_stream(&self, request: LLMRequest) -> Result<String, LLMError>;
    fn provider_name(&self) -> &'static str;

    fn as_any(&self) -> &dyn std::any::Any;
}

#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub openai: Option<OpenAIConfig>,
    pub claude: Option<ClaudeConfig>,
    pub google: Option<GoogleConfig>,
}

#[derive(Debug, Clone)]
pub struct OpenAIConfig {
    pub api_key: String,
    pub base_url: Option<String>,
    pub model: String,
}

#[derive(Debug, Clone)]
pub struct ClaudeConfig {
    pub api_key: String,
    pub model: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GoogleConfig {
    pub api_key: String,
    pub model: String,
    pub base_url: Option<String>,
}
