//! 集成测试文件 - 测试新架构与现有代码的集成

use axum::{
    response::Json as JsonResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use omni_agent::{
    core::{
        router::IntelligentRouter,
        state::{StateManager, BufferedMessage, MessageType},
    },
    services::llm::LLMService,
};

/// 请求消息结构
#[derive(Debug, Serialize, Deserialize)]
struct UserRequest {
    message: String,
    context: Option<HashMap<String, serde_json::Value>>,
}

/// 响应消息结构
#[derive(Debug, Serialize, Deserialize)]
struct AgentResponse {
    message: String,
    source: String,
    details: HashMap<String, serde_json::Value>,
}

/// 应用状态
#[derive(Clone)]
struct AppState {
    router: IntelligentRouter,
    state_manager: StateManager,
    llm_service: LLMService,
}

/// 处理聊天请求
async fn chat_handler(
    State(state): State<AppState>,
    Json(request): Json<UserRequest>,
) -> JsonResponse<AgentResponse> {
    info!("处理用户请求: {}", request.message);
    
    // 1. 首先获取对话缓冲区中的历史消息
    let buffer_messages = state.state_manager.get_buffer_messages().await;
    info!("缓冲区中找到 {} 条历史消息", buffer_messages.len());
    
    // 2. 使用智能路由器决定处理方式
    let decision = state.router.route_message(&request.message).await;
    
    // 3. 根据决策处理请求
    let (response_message, source) = match decision.target {
        omni_agent::core::router::RouteTarget::LocalLLM => {
            // 使用LLM服务处理，传入缓冲区消息作为上下文
            let context_messages: Vec<_> = buffer_messages
                .iter()
                .map(|msg| msg.content.clone())
                .collect();
            
            match state.llm_service.process_message(&request.message, &context_messages).await {
                Ok((response, _token_usage)) => (response, "local_llm".to_string()),
                Err(e) => (format!("处理失败: {}", e), "error".to_string()),
            }
        }
        omni_agent::core::router::RouteTarget::A2AAgent(agent_name) => {
            (format!("将路由到A2A智能体: {}", agent_name), "a2a_agent".to_string())
        }
        omni_agent::core::router::RouteTarget::MCPTool(tool_name) => {
            (format!("将路由到MCP工具: {}", tool_name), "mcp_tool".to_string())
        }
    };
    
    // 4. 将用户消息添加到对话缓冲区
    let buffered_message = BufferedMessage {
        id: uuid::Uuid::new_v4(),
        content: request.message.clone(),
        timestamp: chrono::Utc::now(),
        message_type: MessageType::UserMessage,
        context_relevance: 0.9,
    };
    
    if let Err(e) = state.state_manager.add_to_buffer(buffered_message).await {
        info!("添加消息到缓冲区失败: {}", e);
    }
    
    // 5. 将响应也添加到缓冲区
    let response_buffered_message = BufferedMessage {
        id: uuid::Uuid::new_v4(),
        content: response_message.clone(),
        timestamp: chrono::Utc::now(),
        message_type: MessageType::LLMResponse,
        context_relevance: 0.8,
    };
    
    if let Err(e) = state.state_manager.add_to_buffer(response_buffered_message).await {
        info!("添加响应到缓冲区失败: {}", e);
    }
    
    JsonResponse(AgentResponse {
        message: response_message,
        source,
        details: HashMap::new(),
    })
}

/// 健康检查端点
async fn health_handler() -> JsonResponse<serde_json::Value> {
    JsonResponse(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// 获取状态信息
async fn status_handler(State(state): State<AppState>) -> JsonResponse<serde_json::Value> {
    JsonResponse(json!({
        "buffer_size": state.state_manager.buffer_size(),
        "router_status": "active",
        "llm_service": "mock_mode",
        "buffer_messages": state.state_manager.get_buffer_messages().await.len()
    }))
}

use axum::extract::State;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    
    info!("🚀 启动集成测试服务器...");
    
    // 创建核心组件
    let router = IntelligentRouter::new();
    let state_manager = StateManager::new();
    let llm_service = LLMService::new(true); // 使用模拟模式
    
    // 创建应用状态
    let state = AppState {
        router,
        state_manager,
        llm_service,
    };
    
    // 创建路由
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/status", get(status_handler))
        .route("/chat", post(chat_handler))
        .with_state(state);
    
    let addr = "127.0.0.1:3000";
    info!("🌐 服务器启动于 http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}