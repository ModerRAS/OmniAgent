use serde::{Deserialize, Serialize};
use crate::protocol::manifest::A2AManifest;
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum A2AError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Protocol error: {0}")]
    Protocol(String),
}


#[derive(Debug, Clone)]
pub struct A2AClient {
    pub base_url: String,
    pub client: reqwest::Client,
}

impl A2AClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_manifest(&self) -> Result<crate::protocol::manifest::A2AManifest, A2AError> {
        let url = format!("{}/manifest", self.base_url);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(A2AError::Protocol(
                format!("Failed to fetch manifest: {}", response.status())
            ));
        }

        let manifest: A2AManifest = response.json().await?;
        Ok(manifest)
    }

    pub async fn send_message(
        &self,
        message: A2AMessage,
    ) -> Result<A2AMessage, A2AError> {
        let url = format!("{}/messages", self.base_url);
        
        let response = self.client
            .post(&url)
            .json(&message)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(A2AError::Protocol(
                format!("Failed to send message: {}", response.status())
            ));
        }

        let response_message: A2AMessage = response.json().await?;
        Ok(response_message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2AMessage {
    pub id: Uuid,
    pub sender: String,
    pub recipient: String,
    pub content: A2AContent,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum A2AContent {
    Text { text: String },
    Task { 
        task_type: String,
        parameters: HashMap<String, serde_json::Value>,
    },
    Response { 
        status: String,
        data: serde_json::Value,
    },
    Error { 
        code: String,
        message: String,
    },
}