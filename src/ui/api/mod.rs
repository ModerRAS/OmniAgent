//! REST API模块 - 简化版

use axum::{
    response::Json as JsonResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

/// 请求消息结构
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    pub message: String,
    pub context: Option<HashMap<String, serde_json::Value>>,
}

/// 响应消息结构
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentResponse {
    pub message: String,
    pub source: String,
    pub details: HashMap<String, serde_json::Value>,
}

/// 对话缓冲区消息结构
#[derive(Debug, Serialize, Deserialize)]
pub struct BufferRequest {
    pub action: String,
    pub message: Option<String>,
}

/// 对话缓冲区响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct BufferResponse {
    pub status: String,
    pub message: Option<String>,
}

/// 简化的应用状态
#[derive(Clone)]
pub struct AppState {
    pub name: String,
}

/// 处理聊天请求
pub async fn chat_handler(
    State(_state): State<AppState>,
    Json(request): Json<UserRequest>,
) -> JsonResponse<AgentResponse> {
    JsonResponse(AgentResponse {
        message: format!("处理消息: {}", request.message),
        source: "local_llm".to_string(),
        details: HashMap::new(),
    })
}

/// 健康检查端点
pub async fn health_handler() -> JsonResponse<serde_json::Value> {
    JsonResponse(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// 对话缓冲区管理端点
pub async fn buffer_handler(
    State(_state): State<AppState>,
    Json(request): Json<BufferRequest>,
) -> JsonResponse<BufferResponse> {
    JsonResponse(BufferResponse {
        status: "success".to_string(),
        message: Some(format!("处理了缓冲区请求: {}", request.action)),
    })
}

/// 创建API路由
pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_handler))
        .route("/chat", post(chat_handler))
        .route("/buffer", post(buffer_handler))
}

use axum::extract::State;