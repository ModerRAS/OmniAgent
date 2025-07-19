use crate::agent::{Agent, AgentConfig};
use crate::a2a::client::A2AClient;
use crate::mcp::client::MCPClient;
use std::collections::HashMap;

pub struct AgentBuilder {
    config: AgentConfig,
    mcp_endpoints: HashMap<String, String>,
    a2a_endpoints: HashMap<String, String>,
}

impl AgentBuilder {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            config: AgentConfig {
                name: name.to_string(),
                description: description.to_string(),
                version: "0.1.0".to_string(),
            },
            mcp_endpoints: HashMap::new(),
            a2a_endpoints: HashMap::new(),
        }
    }

    pub fn version(mut self, version: &str) -> Self {
        self.config.version = version.to_string();
        self
    }

    pub fn add_mcp(mut self, name: &str, url: &str) -> Self {
        self.mcp_endpoints.insert(name.to_string(), url.to_string());
        self
    }

    pub fn add_a2a(mut self, name: &str, url: &str) -> Self {
        self.a2a_endpoints.insert(name.to_string(), url.to_string());
        self
    }

    pub async fn build(mut self) -> Result<Agent, String> {
        let mut agent = Agent::new(self.config);

        // Add MCP clients
        for (name, url) in self.mcp_endpoints {
            let client = MCPClient::new(url);
            agent.add_mcp_client(name, client).await?;
        }

        // Add A2A clients
        for (name, url) in self.a2a_endpoints {
            let client = A2AClient::new(url);
            agent.add_a2a_client(name, client).await?;
        }

        // Fetch all manifests
        agent.fetch_manifests().await?;

        Ok(agent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};

    #[tokio::test]
    async fn test_agent_builder() {
        let mock_mcp_server = MockServer::start().await;
        let mock_a2a_server = MockServer::start().await;

        // Mock MCP manifest endpoint
        Mock::given(method("GET"))
            .and(path("/manifest"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "test-mcp",
                "version": "1.0.0",
                "description": "Test MCP server",
                "capabilities": vec!["tool1", "tool2"],
                "tools": []
            })))
            .mount(&mock_mcp_server)
            .await;

        // Mock A2A manifest endpoint
        Mock::given(method("GET"))
            .and(path("/manifest"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "test-a2a",
                "version": "1.0.0",
                "description": "Test A2A server",
                "capabilities": vec!["cap1", "cap2"],
                "supported_protocols": vec!["http"],
                "endpoints": vec!["http://localhost:8080"]
            })))
            .mount(&mock_a2a_server)
            .await;

        let agent = AgentBuilder::new("test-agent", "A test agent")
            .version("1.0.0")
            .add_mcp("mcp1", &format!("{}/", mock_mcp_server.uri()))
            .add_a2a("a2a1", &format!("{}/", mock_a2a_server.uri()))
            .build()
            .await
            .unwrap();

        assert_eq!(agent.config.name, "test-agent");
        assert_eq!(agent.config.version, "1.0.0");
        assert_eq!(agent.mcp_clients.len(), 1);
        assert_eq!(agent.a2a_clients.len(), 1);
    }
}