use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCard {
    pub capabilities: AgentCapabilities,
    pub default_input_modes: Vec<String>,
    pub default_output_modes: Vec<String>,
    pub description: String,
    pub name: String,
    pub skills: Vec<AgentSkill>,
    pub url: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<AgentProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_schemes: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_authenticated_extended_card: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_notifications: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_transition_history: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSkill {
    pub description: String,
    pub id: String,
    pub name: String,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_modes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_modes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProvider {
    pub organization: String,
    pub url: String,
}

impl AgentCard {
    pub fn new(
        name: String,
        description: String,
        version: String,
        url: String,
        skills: Vec<AgentSkill>,
    ) -> Self {
        Self {
            capabilities: AgentCapabilities {
                push_notifications: Some(false),
                state_transition_history: Some(true),
                streaming: Some(false),
            },
            default_input_modes: vec!["text/plain".to_string(), "application/json".to_string()],
            default_output_modes: vec!["text/plain".to_string(), "application/json".to_string()],
            description,
            name,
            skills,
            url,
            version,
            documentation_url: None,
            provider: Some(AgentProvider {
                organization: "OmniAgent".to_string(),
                url: "https://github.com/omni-agent".to_string(),
            }),
            security: None,
            security_schemes: None,
            supports_authenticated_extended_card: Some(false),
        }
    }

    pub fn with_capabilities(mut self, capabilities: AgentCapabilities) -> Self {
        self.capabilities = capabilities;
        self
    }

    pub fn with_input_modes(mut self, input_modes: Vec<String>) -> Self {
        self.default_input_modes = input_modes;
        self
    }

    pub fn with_output_modes(mut self, output_modes: Vec<String>) -> Self {
        self.default_output_modes = output_modes;
        self
    }

    pub fn with_documentation_url(mut self, url: String) -> Self {
        self.documentation_url = Some(url);
        self
    }
}