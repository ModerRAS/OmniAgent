//! 使用Mock服务器的API集成测试

use crate::mock_servers::{start_mock_a2a_server, start_mock_mcp_server};
use reqwest;
use serde_json::json;
use tokio;

#[tokio::test]
async fn test_api_with_mock_a2a_server() {
    // 启动Mock A2A服务器
    let a2a_server_url = start_mock_a2a_server().await.unwrap();
    
    // 使用reqwest客户端测试API调用
    let client = reqwest::Client::new();
    
    // 测试健康检查端点
    let health_response = client
        .get(&format!("{}/health", a2a_server_url))
        .send()
        .await
        .unwrap();
    
    assert_eq!(health_response.status(), 200);
    
    let health_data: serde_json::Value = health_response.json().await.unwrap();
    assert_eq!(health_data["status"], "healthy");
    assert_eq!(health_data["service"], "mock-a2a-server");
    
    // 测试消息发送端点
    let message_data = json!({
        "id": "integration-test-1",
        "content": "集成测试消息"
    });
    
    let message_response = client
        .post(&format!("{}/messages", a2a_server_url))
        .json(&message_data)
        .send()
        .await
        .unwrap();
    
    assert_eq!(message_response.status(), 200);
    
    let message_result: serde_json::Value = message_response.json().await.unwrap();
    assert_eq!(message_result["message_id"], "integration-test-1");
    assert!(message_result["response"].as_str().unwrap().contains("Mock A2A响应"));
    assert_eq!(message_result["status"], "success");
}

#[tokio::test]
async fn test_api_with_mock_mcp_server() {
    // 启动Mock MCP服务器
    let mcp_server_url = start_mock_mcp_server().await.unwrap();
    
    // 使用reqwest客户端测试API调用
    let client = reqwest::Client::new();
    
    // 测试健康检查端点
    let health_response = client
        .get(&format!("{}/health", mcp_server_url))
        .send()
        .await
        .unwrap();
    
    assert_eq!(health_response.status(), 200);
    
    let health_data: serde_json::Value = health_response.json().await.unwrap();
    assert_eq!(health_data["status"], "healthy");
    assert_eq!(health_data["service"], "mock-mcp-server");
    
    // 测试MCP调用端点
    let mcp_data = json!({
        "method": "integration.test",
        "params": {
            "test_param": "test_value"
        },
        "id": 456
    });
    
    let mcp_response = client
        .post(&format!("{}/mcp", mcp_server_url))
        .json(&mcp_data)
        .send()
        .await
        .unwrap();
    
    assert_eq!(mcp_response.status(), 200);
    
    let mcp_result: serde_json::Value = mcp_response.json().await.unwrap();
    assert_eq!(mcp_result["id"], 456);
    assert_eq!(mcp_result["jsonrpc"], "2.0");
    assert!(mcp_result["result"].as_str().unwrap().contains("Mock MCP响应"));
}

#[tokio::test]
async fn test_concurrent_mock_server_requests() {
    // 启动多个Mock服务器实例
    let a2a_server_url = start_mock_a2a_server().await.unwrap();
    let mcp_server_url = start_mock_mcp_server().await.unwrap();
    
    // 并发发送多个请求
    let client = reqwest::Client::new();
    
    let mut handles = vec![];
    
    // 发送多个A2A请求
    for i in 0..5 {
        let client = client.clone();
        let url = a2a_server_url.clone();
        let handle = tokio::spawn(async move {
            let message_data = json!({
                "id": format!("concurrent-a2a-{}", i),
                "content": format!("并发A2A消息 {}", i)
            });
            
            let response = client
                .post(&format!("{}/messages", url))
                .json(&message_data)
                .send()
                .await
                .unwrap();
            
            response.status()
        });
        handles.push(handle);
    }
    
    // 发送多个MCP请求
    for i in 0..5 {
        let client = client.clone();
        let url = mcp_server_url.clone();
        let handle = tokio::spawn(async move {
            let mcp_data = json!({
                "method": format!("concurrent.method.{}", i),
                "params": {},
                "id": i
            });
            
            let response = client
                .post(&format!("{}/mcp", url))
                .json(&mcp_data)
                .send()
                .await
                .unwrap();
            
            response.status()
        });
        handles.push(handle);
    }
    
    // 等待所有请求完成
    let results: Vec<_> = futures::future::join_all(handles).await;
    
    // 验证所有请求都成功
    for result in results {
        let status = result.unwrap();
        assert_eq!(status, 200);
    }
}

#[tokio::test]
async fn test_mock_server_error_handling() {
    // 启动Mock服务器
    let a2a_server_url = start_mock_a2a_server().await.unwrap();
    
    // 测试向不存在的端点发送请求
    let client = reqwest::Client::new();
    let error_response = client
        .get(&format!("{}/nonexistent", a2a_server_url))
        .send()
        .await
        .unwrap();
    
    // Mock服务器没有处理这个路由，应该返回404
    assert_eq!(error_response.status(), 404);
}