use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum MCPError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Protocol error: {0}")]
    Protocol(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub capabilities: Vec<String>,
    pub tools: Vec<MCPTool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    pub output_schema: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct MCPClient {
    pub base_url: String,
    pub client: reqwest::Client,
}

impl MCPClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_manifest(&self
    ) -> Result<MCPManifest, MCPError> {
        let url = format!("{}/manifest", self.base_url);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(MCPError::Protocol(
                format!("Failed to fetch manifest: {}", response.status())
            ));
        }

        let manifest: MCPManifest = response.json().await?;
        Ok(manifest)
    }

    pub async fn call_tool(
        &self,
        tool_name: &str,
        parameters: serde_json::Value,
    ) -> Result<serde_json::Value, MCPError> {
        let url = format!("{}/tools/{}/call", self.base_url, tool_name);
        
        #[derive(Serialize)]
        struct ToolCallRequest {
            parameters: serde_json::Value,
            id: Uuid,
        }

        let request = ToolCallRequest {
            parameters,
            id: Uuid::new_v4(),
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(MCPError::Protocol(
                format!("Tool call failed: {}", response.status())
            ));
        }

        let result: serde_json::Value = response.json().await?;
        Ok(result)
    }
}