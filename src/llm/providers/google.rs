use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::llm::providers::{LLMProvider, LLMRequest, LLMResponse, LLMError, MessageRole as OurMessageRole, Usage};

#[derive(Debug, Clone)]
pub struct GoogleProvider {
    api_key: String,
    model: String,
    base_url: String,
}

impl GoogleProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            api_key,
            model: model.unwrap_or_else(|| "gemini-pro".to_string()),
            base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
        }
    }

    fn convert_message_role(role: OurMessageRole) -> String {
        match role {
            OurMessageRole::System => "system".to_string(),
            OurMessageRole::User => "user".to_string(),
            OurMessageRole::Assistant => "model".to_string(),
        }
    }

    fn convert_messages(&self, messages: Vec<crate::llm::providers::Message>) -> (Option<String>, Vec<GeminiMessage>) {
        let mut system_message = None;
        let mut gemini_messages = Vec::new();

        for msg in messages {
            match msg.role {
                OurMessageRole::System => {
                    system_message = Some(msg.content);
                }
                OurMessageRole::User => {
                    gemini_messages.push(GeminiMessage {
                        role: "user".to_string(),
                        parts: vec![GeminiPart {
                            text: Some(msg.content),
                            inline_data: None,
                        }],
                    });
                }
                OurMessageRole::Assistant => {
                    gemini_messages.push(GeminiMessage {
                        role: "model".to_string(),
                        parts: vec![GeminiPart {
                            text: Some(msg.content),
                            inline_data: None,
                        }],
                    });
                }
            }
        }

        (system_message, gemini_messages)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiRequestBody {
    contents: Vec<GeminiMessage>,
    system_instruction: Option<GeminiSystemInstruction>,
    generation_config: Option<GeminiGenerationConfig>,
    safety_settings: Option<Vec<GeminiSafetySetting>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiMessage {
    role: String,
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiPart {
    text: Option<String>,
    inline_data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiSystemInstruction {
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiGenerationConfig {
    temperature: Option<f32>,
    top_p: Option<f32>,
    top_k: Option<i32>,
    max_output_tokens: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiSafetySetting {
    category: String,
    threshold: String,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
    usage_metadata: Option<GeminiUsageMetadata>,
    model_version: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidate {
    content: Option<GeminiContent>,
    finish_reason: Option<String>,
    index: Option<i32>,
    safety_ratings: Option<Vec<GeminiSafetyRating>>,
}

#[derive(Debug, Deserialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
    role: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GeminiSafetyRating {
    category: String,
    probability: String,
}

#[derive(Debug, Deserialize)]
struct GeminiUsageMetadata {
    prompt_token_count: Option<u32>,
    candidates_token_count: Option<u32>,
    total_token_count: Option<u32>,
}

#[async_trait]
impl LLMProvider for GoogleProvider {
    async fn chat(&self, request: LLMRequest) -> Result<LLMResponse, LLMError> {
        let client = reqwest::Client::new();
        
        let (system_message, gemini_messages) = self.convert_messages(request.messages);
        
        let mut request_body = GeminiRequestBody {
            contents: gemini_messages,
            system_instruction: None,
            generation_config: Some(GeminiGenerationConfig {
                temperature: request.temperature,
                top_p: None,
                top_k: None,
                max_output_tokens: request.max_tokens.map(|v| v as i32),
            }),
            safety_settings: None,
        };

        // 添加系统指令
        if let Some(system_msg) = system_message {
            request_body.system_instruction = Some(GeminiSystemInstruction {
                parts: vec![GeminiPart {
                    text: Some(system_msg),
                    inline_data: None,
                }],
            });
        }

        let url = format!("{}/models/{}:generateContent", self.base_url, self.model);
        
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .query(&[("key", &self.api_key)])
            .json(&request_body)
            .send()
            .await
            .map_err(|e| LLMError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(LLMError::ApiError(format!("Google AI API error: {}", error_text)));
        }

        let response_data: GeminiResponse = response
            .json()
            .await
            .map_err(|e| LLMError::ApiError(format!("Parse error: {}", e)))?;

        let candidate = response_data.candidates
            .get(0)
            .ok_or_else(|| LLMError::ApiError("No response from Google AI".to_string()))?;

        let content = candidate.content.as_ref()
            .and_then(|content| content.parts.get(0))
            .and_then(|part| part.text.clone())
            .unwrap_or_default();

        let usage = response_data.usage_metadata.as_ref().map(|usage| Usage {
            prompt_tokens: usage.prompt_token_count.unwrap_or(0),
            completion_tokens: usage.candidates_token_count.unwrap_or(0),
            total_tokens: usage.total_token_count.unwrap_or(0),
        });

        Ok(LLMResponse {
            content,
            model: response_data.model_version.unwrap_or_else(|| self.model.clone()),
            usage,
        })
    }

    async fn chat_stream(
        &self,
        _request: LLMRequest,
    ) -> Result<String, LLMError> {
        Err(LLMError::ApiError("Google AI streaming not implemented yet".to_string()))
    }

    fn provider_name(&self) -> &'static str {
        "google"
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_provider_creation() {
        let provider = GoogleProvider::new(
            "test-key".to_string(),
            Some("gemini-pro-vision".to_string()),
        );
        
        assert_eq!(provider.provider_name(), "google");
        assert_eq!(provider.model, "gemini-pro-vision");
        assert_eq!(provider.base_url, "https://generativelanguage.googleapis.com/v1beta");
    }

    #[test]
    fn test_message_conversion() {
        let provider = GoogleProvider::new("test-key".to_string(), None);
        
        let messages = vec![
            crate::llm::providers::Message {
                role: OurMessageRole::System,
                content: "You are a helpful assistant".to_string(),
            },
            crate::llm::providers::Message {
                role: OurMessageRole::User,
                content: "Hello, Gemini!".to_string(),
            },
        ];

        let (system_msg, gemini_msgs) = provider.convert_messages(messages);
        
        assert_eq!(system_msg, Some("You are a helpful assistant".to_string()));
        assert_eq!(gemini_msgs.len(), 1);
        assert_eq!(gemini_msgs[0].role, "user");
        assert_eq!(gemini_msgs[0].parts[0].text, Some("Hello, Gemini!".to_string()));
    }
}