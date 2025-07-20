use serde::Serialize;
use crate::protocol::manifest::MCPManifest;
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
    ) -> Result<crate::protocol::manifest::MCPManifest, MCPError> {
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