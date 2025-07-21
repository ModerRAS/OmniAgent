pub mod manager;
pub mod providers;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::llm::manager::LLMManager;
use crate::llm::providers::{LLMRequest, ProviderConfig};
use crate::protocol::message::{Message, MessageContent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub provider: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub use_mock: bool,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            provider: "openai".to_string(),
            model: "gpt-3.5-turbo".to_string(),
            temperature: 0.7,
            max_tokens: 1000,
            use_mock: true,
        }
    }
}

pub struct LLMService {
    pub manager: LLMManager,
    pub config: LLMConfig,
}

impl LLMService {
    pub fn new(config: LLMConfig, provider_config: ProviderConfig) -> Self {
        let manager = LLMManager::new(provider_config, &config.provider);
        Self { manager, config }
    }

    pub async fn from_config(
        config: &crate::config::LLMConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let provider_config = ProviderConfig {
            openai: if config.provider == "openai" {
                Some(crate::llm::providers::OpenAIConfig {
                    api_key: config.api_key.clone(),
                    base_url: Some(
                        config
                            .base_url
                            .clone()
                            .unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
                    ),
                    model: config.model.clone(),
                })
            } else {
                None
            },
            claude: if config.provider == "claude" {
                Some(crate::llm::providers::ClaudeConfig {
                    api_key: config.api_key.clone(),
                    model: config.model.clone(),
                })
            } else {
                None
            },
            google: if config.provider == "google" {
                Some(crate::llm::providers::GoogleConfig {
                    api_key: config.api_key.clone(),
                    model: config.model.clone(),
                })
            } else {
                None
            },
        };

        let llm_config = LLMConfig {
            provider: config.provider.clone(),
            model: config.model.clone(),
            temperature: config.temperature,
            max_tokens: config.max_tokens,
            use_mock: config.use_mock,
        };

        Ok(Self::new(llm_config, provider_config))
    }

    pub async fn process_message(
        &self,
        input: &str,
        context: &[Message],
    ) -> Result<Message, String> {
        if self.config.use_mock {
            self.mock_process(input, context).await
        } else {
            self.real_process(input, context).await
        }
    }

    async fn real_process(&self, input: &str, _context: &[Message]) -> Result<Message, String> {
        let provider = self
            .manager
            .get_provider(&self.config.provider)
            .await
            .ok_or_else(|| format!("Provider {} not found", self.config.provider))?;

        let messages = vec![crate::llm::providers::Message {
            role: crate::llm::providers::MessageRole::User,
            content: input.to_string(),
        }];

        let request = LLMRequest {
            messages,
            model: self.config.model.clone(),
            temperature: Some(self.config.temperature),
            max_tokens: Some(self.config.max_tokens),
            stream: Some(false),
        };

        match provider.chat(request).await {
            Ok(response) => Ok(Message::new(
                "llm".to_string(),
                "user".to_string(),
                MessageContent::Text {
                    text: response.content,
                },
                Some(json!({
                    "provider": self.config.provider,
                    "model": response.model,
                    "usage": response.usage
                })),
            )),
            Err(e) => Err(format!("LLM error: {}", e)),
        }
    }

    async fn mock_process(&self, input: &str, _context: &[Message]) -> Result<Message, String> {
        // Enhanced mock processing with provider awareness
        let response_text = match self.config.provider.as_str() {
            "openai" => format!("[OpenAI {}] Processing: {}", self.config.model, input),
            "claude" => format!("[Claude {}] Analyzing: {}", self.config.model, input),
            "google" => format!("[Google {}] Responding: {}", self.config.model, input),
            _ => format!("[Mock] Received: {}", input),
        };

        let content = if input.contains("tool") {
            MessageContent::ToolCall {
                tool: "mock_tool".to_string(),
                parameters: json!({"query": input, "mock": true}),
            }
        } else if input.contains("agent") {
            MessageContent::AgentRequest {
                request_type: "process".to_string(),
                payload: json!({"input": input, "mock": true}),
            }
        } else {
            MessageContent::Text {
                text: response_text,
            }
        };

        Ok(Message::new(
            "llm".to_string(),
            "user".to_string(),
            content,
            Some(json!({
                "provider": &self.config.provider,
                "model": &self.config.model,
                "mock": true
            })),
        ))
    }

    pub async fn list_providers(&self) -> Vec<String> {
        self.manager.list_providers().await
    }

    pub fn get_current_provider(&self) -> &str {
        &self.config.provider
    }
}
