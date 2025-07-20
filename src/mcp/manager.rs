use std::collections::HashMap;
use std::time::Duration;
use serde_json::Value;
use tracing::{info, error, debug};

use crate::config::{McpConfig, McpServerConfig};
use crate::mcp::client::McpClient;

#[derive(Debug, Clone)]
pub struct ConnectedMcpServer {
    pub config: McpServerConfig,
    pub client: McpClient,
    pub connected: bool,
    pub capabilities: Vec<String>,
}

pub struct McpManager {
    config: McpConfig,
    servers: HashMap<String, ConnectedMcpServer>,
}

impl McpManager {
    pub async fn from_config(config: &McpConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let mut manager = Self {
            config: config.clone(),
            servers: HashMap::new(),
        };

        for (id, server_config) in &config.servers {
            if server_config.enabled {
                manager.add_server(id.clone(), server_config.clone())?;
            }
        }

        Ok(manager)
    }

    pub fn add_server(
        &mut self,
        id: String,
        config: McpServerConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = McpClient::new(config.url.clone());

        let server = ConnectedMcpServer {
            config,
            client,
            connected: false,
            capabilities: Vec::new(),
        };

        self.servers.insert(id, server);
        Ok(())
    }

    pub async fn connect_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let server_ids: Vec<String> = self.servers.keys().cloned().collect();
        
        for id in server_ids {
            if let Err(e) = self.connect_server(&id).await {
                error!("Failed to connect to MCP server {}: {}", id, e);
            }
        }

        Ok(())
    }

    pub async fn connect_server(&mut self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(server) = self.servers.get_mut(id) {
            info!("Connecting to MCP server: {}", server.config.name);
            
            match server.client.fetch_manifest().await {
                Ok(manifest) => {
                    server.connected = true;
                    server.capabilities = manifest.capabilities;
                    info!("Connected to MCP server {} with capabilities: {:?}", 
                          server.config.name, server.capabilities);
                }
                Err(e) => {
                    server.connected = false;
                    error!("Failed to connect to MCP server {}: {}", server.config.name, e);
                    return Err(e.into());
                }
            }
        }
        
        Ok(())
    }

    pub async fn disconnect_all(&mut self) {
        for (_, server) in &mut self.servers {
            if server.connected {
                server.connected = false;
                info!("Disconnected from MCP server: {}", server.config.name);
            }
        }
    }

    pub fn get_connected_servers(&self) -> Vec<String> {
        self.servers
            .iter()
            .filter(|(_, server)| server.connected)
            .map(|(id, _)| id.clone())
            .collect()
    }

    pub fn get_server_capabilities(&self, id: &str) -> Option<&Vec<String>> {
        self.servers.get(id).map(|server| &server.capabilities)
    }

    pub async fn call_tool(
        &self,
        server_id: &str,
        tool_name: &str,
        parameters: Value,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        if let Some(server) = self.servers.get(server_id) {
            if server.connected {
                server.client.call_tool(tool_name, parameters).await
            } else {
                Err(format!("MCP server {} is not connected", server_id).into())
            }
        } else {
            Err(format!("MCP server {} not found", server_id).into())
        }
    }

    pub fn list_tools(&self) -> HashMap<String, Vec<String>> {
        let mut tools = HashMap::new();
        
        for (id, server) in &self.servers {
            if server.connected {
                tools.insert(id.clone(), server.capabilities.clone());
            }
        }
        
        tools
    }
}