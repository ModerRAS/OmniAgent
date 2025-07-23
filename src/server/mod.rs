use axum::{
    extract::{Path, State},
    response::Response,
    routing::get,
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::agent::{Agent, AgentConfig};
use crate::protocol::message::{Message, MessageContent};
use crate::protocol::agent_card::AgentCard;

#[derive(Clone)]
pub struct AppState {
    pub agent: Arc<RwLock<Agent>>,
}

impl AppState {
    pub fn new(agent: Agent) -> Self {
        Self {
            agent: Arc::new(RwLock::new(agent)),
        }
    }
}

pub struct A2AServer {
    pub port: u16,
    state: AppState,
}

impl A2AServer {
    pub fn new(port: u16) -> Self {
        let agent_config = AgentConfig {
            name: "OmniAgent".to_string(),
            description: "A2A + MCP Agent Server with LLM integration".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        let agent = Agent::new(agent_config);
        let state = AppState::new(agent);

        Self { port, state }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let app = Router::new()
            .route("/", get(root))
            .route("/health", get(health))
            .route("/manifest", get(get_manifest))
            .route("/agent.json", get(get_agent_card))
            .route("/messages", axum::routing::post(handle_message))
            .route("/messages/:id", get(get_message))
            .with_state(self.state);

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;

        println!("🔥 A2A Server running on http://localhost:{}", self.port);

        axum::serve(listener, app).await?;
        Ok(())
    }
}

async fn root() -> Json<serde_json::Value> {
    Json(json!({
        "name": "OmniAgent A2A Server",
        "version": env!("CARGO_PKG_VERSION").to_string(),
        "description": "A2A + MCP protocol implementation in Rust",
        "endpoints": {
            "/health": "Health check",
            "/manifest": "Agent capabilities",
            "/agent.json": "Agent card (A2A specification)",
            "/messages": "Send messages",
            "/messages/:id": "Get message by ID"
        }
    }))
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn get_manifest(State(state): State<AppState>) -> Json<serde_json::Value> {
    let agent = state.agent.read().await;
    let capabilities = agent.get_capabilities().await;

    Json(json!({
        "name": agent.config.name,
        "version": agent.config.version,
        "description": agent.config.description,
        "capabilities": capabilities,
        "supported_protocols": vec!["a2a", "mcp"],
        "endpoints": vec!["http", "websocket"]
    }))
}

async fn handle_message(
    State(state): State<AppState>,
    Json(message): Json<Message>,
) -> Result<Json<Message>, Response> {
    let agent = state.agent.read().await;

    let response_content = match &message.content {
        MessageContent::Text { text } => MessageContent::Text {
            text: format!("Received: {text}"),
        },
        MessageContent::ToolCall { tool, parameters } => MessageContent::ToolResult {
            tool: tool.clone(),
            result: json!({"mock": true, "tool": tool, "parameters": parameters}),
        },
        MessageContent::AgentRequest {
            request_type,
            payload,
        } => MessageContent::Text {
            text: format!("Received {request_type} request with payload: {payload}"),
        },
        _ => MessageContent::Error {
            code: "UNSUPPORTED".to_string(),
            message: "Unsupported message type".to_string(),
        },
    };

    let response = Message::new(
        agent.config.name.clone(),
        message.sender.clone(),
        response_content,
        None,
    );

    Ok(Json(response))
}

async fn get_agent_card(State(state): State<AppState>) -> Json<AgentCard> {
    let agent = state.agent.read().await;
    // Construct base URL from the server configuration
    // In a real deployment, this would come from configuration
    let base_url = "http://localhost:8080".to_string();
    let card = agent.get_agent_card(base_url).await;
    Json(card)
}

async fn get_message(Path(id): Path<Uuid>) -> Result<Json<Message>, Response> {
    let message = Message::new(
        "server".to_string(),
        "client".to_string(),
        MessageContent::Text {
            text: format!("Message {id} not found"),
        },
        None,
    );

    Ok(Json(message))
}
