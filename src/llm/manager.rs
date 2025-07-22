use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::llm::providers::claude::ClaudeProvider;
use crate::llm::providers::google::GoogleProvider;
use crate::llm::providers::{LLMProvider, OpenAIProvider, ProviderConfig};

#[derive(Clone)]
pub struct LLMManager {
    providers: Arc<RwLock<HashMap<String, Box<dyn LLMProvider + Send + Sync>>>>,
    default_provider: String,
}

impl std::fmt::Debug for LLMManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LLMManager")
            .field("default_provider", &self.default_provider)
            .field("providers", &"HashMap<String, Box<dyn LLMProvider>>")
            .finish()
    }
}

impl LLMManager {
    pub fn new(config: ProviderConfig, default_provider: &str) -> Self {
        let mut providers = HashMap::new();

        if let Some(openai_config) = config.openai {
            let provider: Box<dyn LLMProvider + Send + Sync> = Box::new(OpenAIProvider::new(
                openai_config.api_key,
                Some(openai_config.model),
                openai_config.base_url,
            ));
            providers.insert("openai".to_string(), provider);
        }

        if let Some(claude_config) = config.claude {
            let provider: Box<dyn LLMProvider + Send + Sync> = Box::new(ClaudeProvider::new(
                claude_config.api_key,
                Some(claude_config.model),
            ));
            providers.insert("claude".to_string(), provider);
        }

        if let Some(google_config) = config.google {
            let provider: Box<dyn LLMProvider + Send + Sync> = Box::new(GoogleProvider::new(
                google_config.api_key,
                Some(google_config.model),
            ));
            providers.insert("google".to_string(), provider);
        }

        Self {
            providers: Arc::new(RwLock::new(providers)),
            default_provider: default_provider.to_string(),
        }
    }

    pub async fn add_provider(&self, name: String, provider: Box<dyn LLMProvider + Send + Sync>) {
        let mut providers = self.providers.write().await;
        providers.insert(name, provider);
    }

    pub async fn get_provider(&self, name: &str) -> Option<Box<dyn LLMProvider + Send + Sync>> {
        let providers = self.providers.read().await;
        providers.get(name).map(|p| match p.provider_name() {
            "openai" => Box::new(OpenAIProvider::new(
                "mock-key".to_string(),
                Some("gpt-3.5-turbo".to_string()),
                None,
            )) as Box<dyn LLMProvider + Send + Sync>,
            "claude" => Box::new(ClaudeProvider::new(
                "mock-key".to_string(),
                Some("claude-3-haiku-20240307".to_string()),
            )) as Box<dyn LLMProvider + Send + Sync>,
            "google" => Box::new(GoogleProvider::new(
                "mock-key".to_string(),
                Some("gemini-pro".to_string()),
            )) as Box<dyn LLMProvider + Send + Sync>,
            _ => panic!("Unknown provider"),
        })
    }

    pub fn get_default_provider(&self) -> &str {
        &self.default_provider
    }

    pub async fn list_providers(&self) -> Vec<String> {
        let providers = self.providers.read().await;
        providers.keys().cloned().collect()
    }

    pub async fn is_provider_available(&self, name: &str) -> bool {
        let providers = self.providers.read().await;
        providers.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::providers::OpenAIConfig;

    #[tokio::test]
    async fn test_llm_manager_creation() {
        let config = ProviderConfig {
            openai: Some(OpenAIConfig {
                api_key: "test-key".to_string(),
                base_url: None,
                model: "gpt-3.5-turbo".to_string(),
            }),
            claude: None,
            google: None,
        };

        let manager = LLMManager::new(config, "openai");
        let providers = manager.list_providers().await;

        assert_eq!(providers.len(), 1);
        assert!(providers.contains(&"openai".to_string()));
        assert_eq!(manager.get_default_provider(), "openai");
    }

    #[tokio::test]
    async fn test_provider_availability() {
        let config = ProviderConfig {
            openai: Some(OpenAIConfig {
                api_key: "test-key".to_string(),
                base_url: None,
                model: "gpt-3.5-turbo".to_string(),
            }),
            claude: None,
            google: None,
        };

        let manager = LLMManager::new(config, "openai");

        assert!(manager.is_provider_available("openai").await);
        assert!(!manager.is_provider_available("claude").await);
    }
}
