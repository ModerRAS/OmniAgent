use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::a2a::client::A2AClient;
use crate::mcp::client::MCPClient;
use crate::protocol::manifest::Manifest;
use crate::agent::state::StateMachine;
use crate::llm::MockLLM;

pub mod builder;
pub use builder::AgentBuilder;
pub mod state;

#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub name: String,
    pub description: String,
    pub version: String,
}

#[derive(Debug)]
pub struct Agent {
    pub id: Uuid,
    pub config: AgentConfig,
    pub mcp_clients: HashMap<String, MCPClient>,
    pub a2a_clients: HashMap<String, A2AClient>,
    pub manifests: Arc<RwLock<HashMap<String, Manifest>>>,
    pub state_machine: Arc<RwLock<StateMachine>>,
    pub llm: Arc<RwLock<MockLLM>>,
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
            llm: Arc::new(RwLock::new(MockLLM::new(Default::default()))),
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
            let manifest = client.fetch_manifest().await
                .map_err(|e| format!("Failed to fetch MCP manifest for {}: {}", name, e))?;
            self.manifests.write().await.insert(name.clone(), Manifest::MCP(manifest));
        }

        // Fetch A2A manifests
        for (name, client) in &self.a2a_clients {
            let manifest = client.fetch_manifest().await
                .map_err(|e| format!("Failed to fetch A2A manifest for {}: {}", name, e))?;
            self.manifests.write().await.insert(name.clone(), Manifest::A2A(manifest));
        }

        Ok(())
    }

    pub async fn get_capabilities(&self) -> Vec<String> {
        let manifests = self.manifests.read().await;
        manifests.values()
            .flat_map(|manifest| match manifest {
                Manifest::MCP(m) => m.capabilities.clone(),
                Manifest::A2A(m) => m.capabilities.clone(),
            })
            .collect()
    }
}