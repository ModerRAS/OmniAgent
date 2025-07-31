//! Mock服务器实现，用于测试API调用和协议交互

use axum::{
    response::Json as JsonResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// Mock A2A服务器响应结构
#[derive(Debug, Serialize, Deserialize)]
struct MockA2AResponse {
    message_id: String,
    response: String,
    status: String,
}

/// Mock MCP服务器响应结构
#[derive(Debug, Serialize, Deserialize)]
struct MockMCPResponse {
    result: String,
    id: Option<i32>,
    jsonrpc: String,
}

/// 启动Mock A2A服务器
pub async fn start_mock_a2a_server() -> Result<String, Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/health", get(mock_a2a_health))
        .route("/messages", post(mock_a2a_message))
        .route("/messages/:id", get(mock_a2a_get_message));

    let addr = SocketAddr::from(([127, 0, 0, 1], 0)); // 使用端口0让系统分配可用端口
    let listener = TcpListener::bind(addr).await?;
    let addr = listener.local_addr()?;
    
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    Ok(format!("http://{}", addr))
}

/// 启动Mock MCP服务器
pub async fn start_mock_mcp_server() -> Result<String, Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/mcp", post(mock_mcp_call))
        .route("/health", get(mock_mcp_health));

    let addr = SocketAddr::from(([127, 0, 0, 1], 0)); // 使用端口0让系统分配可用端口
    let listener = TcpListener::bind(addr).await?;
    let addr = listener.local_addr()?;
    
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    Ok(format!("http://{}", addr))
}

/// Mock A2A健康检查端点
async fn mock_a2a_health() -> JsonResponse<serde_json::Value> {
    JsonResponse(json!({
        "status": "healthy",
        "service": "mock-a2a-server"
    }))
}

/// Mock A2A消息处理端点
async fn mock_a2a_message(
    axum::Json(payload): axum::Json<serde_json::Value>,
) -> JsonResponse<MockA2AResponse> {
    let message_id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("unknown");
    
    JsonResponse(MockA2AResponse {
        message_id: message_id.to_string(),
        response: format!("Mock A2A响应: 处理了来自{}的消息", message_id),
        status: "success".to_string(),
    })
}

/// Mock A2A获取消息端点
async fn mock_a2a_get_message(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> JsonResponse<serde_json::Value> {
    JsonResponse(json!({
        "id": id,
        "content": format!("这是ID为{}的mock消息内容", id),
        "sender": "mock-sender",
        "recipient": "mock-recipient",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Mock MCP健康检查端点
async fn mock_mcp_health() -> JsonResponse<serde_json::Value> {
    JsonResponse(json!({
        "status": "healthy",
        "service": "mock-mcp-server"
    }))
}

/// Mock MCP工具调用端点
async fn mock_mcp_call(
    axum::Json(payload): axum::Json<serde_json::Value>,
) -> JsonResponse<MockMCPResponse> {
    let method = payload.get("method").and_then(|v| v.as_str()).unwrap_or("unknown");
    let id = payload.get("id").and_then(|v| v.as_i64()).map(|v| v as i32);
    
    JsonResponse(MockMCPResponse {
        result: format!("Mock MCP响应: 执行了{}方法", method),
        id,
        jsonrpc: "2.0".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest;
    use serde_json::json;

    #[tokio::test]
    async fn test_mock_a2a_server() {
        let server_url = start_mock_a2a_server().await.unwrap();
        
        // 测试健康检查
        let client = reqwest::Client::new();
        let health_response = client
            .get(&format!("{}/health", server_url))
            .send()
            .await
            .unwrap();
        
        assert_eq!(health_response.status(), 200);
        
        let health_json: serde_json::Value = health_response.json().await.unwrap();
        assert_eq!(health_json["status"], "healthy");
        assert_eq!(health_json["service"], "mock-a2a-server");
        
        // 测试消息发送
        let message_response = client
            .post(&format!("{}/messages", server_url))
            .json(&json!({
                "id": "test-message-1",
                "content": "测试消息"
            }))
            .send()
            .await
            .unwrap();
        
        assert_eq!(message_response.status(), 200);
        
        let message_result: MockA2AResponse = message_response.json().await.unwrap();
        assert_eq!(message_result.message_id, "test-message-1");
        assert!(message_result.response.contains("Mock A2A响应"));
        assert_eq!(message_result.status, "success");
    }

    #[tokio::test]
    async fn test_mock_mcp_server() {
        let server_url = start_mock_mcp_server().await.unwrap();
        
        // 测试健康检查
        let client = reqwest::Client::new();
        let health_response = client
            .get(&format!("{}/health", server_url))
            .send()
            .await
            .unwrap();
        
        assert_eq!(health_response.status(), 200);
        
        let health_json: serde_json::Value = health_response.json().await.unwrap();
        assert_eq!(health_json["status"], "healthy");
        assert_eq!(health_json["service"], "mock-mcp-server");
        
        // 测试MCP调用
        let mcp_response = client
            .post(&format!("{}/mcp", server_url))
            .json(&json!({
                "method": "test.method",
                "params": {},
                "id": 123
            }))
            .send()
            .await
            .unwrap();
        
        assert_eq!(mcp_response.status(), 200);
        
        let mcp_result: MockMCPResponse = mcp_response.json().await.unwrap();
        assert_eq!(mcp_result.id, Some(123));
        assert_eq!(mcp_result.jsonrpc, "2.0");
        assert!(mcp_result.result.contains("Mock MCP响应"));
    }
}