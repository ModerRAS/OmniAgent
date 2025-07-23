use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::a2a::client::A2AClient;
use crate::agent::state::StateMachine;
use crate::llm::providers::ProviderConfig;
use crate::llm::LLMConfig;
use crate::llm::LLMService;
use crate::mcp::client::MCPClient;
use crate::protocol::manifest::Manifest;
use crate::protocol::agent_card::{AgentCard, AgentSkill};

pub mod builder;
pub use builder::AgentBuilder;
pub mod state;

#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub name: String,
    pub description: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: Uuid,
    pub config: AgentConfig,
    pub mcp_clients: HashMap<String, MCPClient>,
    pub a2a_clients: HashMap<String, A2AClient>,
    pub manifests: Arc<RwLock<HashMap<String, Manifest>>>,
    pub state_machine: Arc<RwLock<StateMachine>>,
    pub llm: Arc<RwLock<LLMService>>,
}

impl Agent {
    pub fn new(config: AgentConfig) -> Self {
        Self {
            id: Uuid::new_v4(),
            config,
            mcp_clients: HashMap::new(),
            a2a_clients: HashMap::new(),
            manifests: Arc::new(RwLock::new(HashMap::new())),
            state_machine: Arc::new(RwLock::new(StateMachine::new(100))),
            llm: Arc::new(RwLock::new(LLMService::new(
                LLMConfig::default(),
                ProviderConfig {
                    openai: None,
                    claude: None,
                    google: None,
                },
            ))),
        }
    }

    pub async fn process_message(
        &self,
        message: crate::protocol::message::Message,
    ) -> Result<crate::protocol::message::Message, String> {
        let mut state_machine = self.state_machine.write().await;
        state_machine.add_message(message.clone());
        state_machine.transition(crate::agent::state::AgentState::Processing);

        // Use LLM to process the message
        let llm = self.llm.read().await;
        let context = state_machine.get_context();

        let response = match message.content {
            crate::protocol::message::MessageContent::Text { ref text } => {
                llm.process_message(text, &context).await?
            }
            _ => {
                // Handle other message types directly
                crate::protocol::message::Message::new(
                    self.config.name.clone(),
                    message.sender.clone(),
                    crate::protocol::message::MessageContent::Text {
                        text: "Processed".to_string(),
                    },
                    None,
                )
            }
        };

        state_machine.add_message(response.clone());
        state_machine.transition(crate::agent::state::AgentState::Idle);

        Ok(response)
    }

    pub async fn add_mcp_client(&mut self, name: String, client: MCPClient) -> Result<(), String> {
        self.mcp_clients.insert(name, client);
        Ok(())
    }

    pub async fn add_a2a_client(&mut self, name: String, client: A2AClient) -> Result<(), String> {
        self.a2a_clients.insert(name, client);
        Ok(())
    }

    pub async fn fetch_manifests(&self) -> Result<(), String> {
        // Fetch MCP manifests
        for (name, client) in &self.mcp_clients {
            let manifest = client
                .fetch_manifest()
                .await
                .map_err(|e| format!("Failed to fetch MCP manifest for {name}: {e}"))?;
            self.manifests
                .write()
                .await
                .insert(name.clone(), Manifest::MCP(manifest));
        }

        // Fetch A2A manifests
        for (name, client) in &self.a2a_clients {
            let manifest = client
                .fetch_manifest()
                .await
                .map_err(|e| format!("Failed to fetch A2A manifest for {name}: {e}"))?;
            self.manifests
                .write()
                .await
                .insert(name.clone(), Manifest::A2A(manifest));
        }

        Ok(())
    }

    pub async fn get_capabilities(&self) -> Vec<String> {
        let manifests = self.manifests.read().await;
        manifests
            .values()
            .flat_map(|manifest| match manifest {
                Manifest::MCP(m) => m.capabilities.clone(),
                Manifest::A2A(m) => m.capabilities.clone(),
            })
            .collect()
    }

    pub async fn get_agent_card(&self, base_url: String) -> AgentCard {
        let capabilities = self.get_capabilities().await;
        
        // Convert capabilities into agent skills, or provide default skills if empty
        let skills = if capabilities.is_empty() {
            vec![
                AgentSkill {
                    id: "text_processing".to_string(),
                    name: "Text Processing".to_string(),
                    description: "Process and respond to text messages".to_string(),
                    tags: vec!["text".to_string(), "processing".to_string()],
                    examples: None,
                    input_modes: None,
                    output_modes: None,
                },
                AgentSkill {
                    id: "llm_integration".to_string(),
                    name: "LLM Integration".to_string(),
                    description: "Use large language models for intelligent responses".to_string(),
                    tags: vec!["llm".to_string(), "ai".to_string()],
                    examples: None,
                    input_modes: None,
                    output_modes: None,
                },
            ]
        } else {
            capabilities
                .iter()
                .map(|capability| AgentSkill {
                    id: capability.to_lowercase().replace(' ', "_"),
                    name: capability.clone(),
                    description: format!("Agent capability: {}", capability),
                    tags: vec![capability.to_lowercase()],
                    examples: None,
                    input_modes: None,
                    output_modes: None,
                })
                .collect()
        };

        AgentCard::new(
            self.config.name.clone(),
            self.config.description.clone(),
            self.config.version.clone(),
            format!("{}/", base_url.trim_end_matches('/')),
            skills,
        )
    }
}
