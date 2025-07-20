use tracing::{info};
use serde_json::json;

use crate::config::AppConfig;
use crate::llm::LLMService;
use crate::server::A2AServer;

pub struct OmniApp {
    config: AppConfig,
}

impl OmniApp {
    pub async fn new(config: AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        info!("🚀 Initializing OmniAgent Application...");

        info!("✅ OmniAgent Application initialized successfully");

        Ok(Self { config })
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🌟 Starting OmniAgent Application...");

        // Initialize LLM service based on config
        let _llm_service = LLMService::from_config(&self.config.llm).await?;

        info!("🎯 OmniAgent is ready!");
        info!("📍 A2A Server: http://{}:{}", self.config.server.host, self.config.server.port);
        info!("📊 MCP Servers configured: {}", self.config.mcp.servers.len());
        info!("🤝 A2A Peers configured: {}", self.config.a2a.servers.len());

        let server = A2AServer::new(self.config.server.port);
        server.run().await?;
        
        Ok(())
    }

    pub async fn health_check(&self) -> serde_json::Value {
        json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "services": {
                "llm": {
                    "provider": self.config.llm.provider,
                    "model": self.config.llm.model,
                    "mock_mode": self.config.llm.use_mock
                },
                "mcp": {
                    "enabled": self.config.mcp.enabled,
                    "total_servers": self.config.mcp.servers.len()
                },
                "a2a": {
                    "enabled": self.config.a2a.enabled,
                    "total_servers": self.config.a2a.servers.len(),
                    "port": self.config.server.port
                }
            }
        })
    }
}