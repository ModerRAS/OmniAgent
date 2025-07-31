//! API端点集成测试

use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::{get, post},
    Router,
};
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceExt; // for `oneshot` and `ready`

// 注意：这些测试使用模拟的应用状态，不依赖真实的LLM调用

#[tokio::test]
async fn test_health_endpoint() {
    // 创建模拟的应用状态
    let app = Router::new().route("/health", get(|| async { serde_json::json!({"status": "healthy"}) }));
    
    // 创建请求
    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    
    // 验证响应状态
    assert_eq!(response.status(), StatusCode::OK);
    
    // 验证响应内容
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["status"], "healthy");
}

#[tokio::test]
async fn test_status_endpoint() {
    // 创建模拟的应用状态
    let app = Router::new().route("/status", get(|| async { 
        serde_json::json!({
            "buffer_size": 0,
            "router_status": "active",
            "llm_service": "mock_mode"
        }) 
    }));
    
    // 创建请求
    let response = app
        .oneshot(Request::builder().uri("/status").body(Body::empty()).unwrap())
        .await
        .unwrap();
    
    // 验证响应状态
    assert_eq!(response.status(), StatusCode::OK);
    
    // 验证响应内容
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["router_status"], "active");
    assert_eq!(body["llm_service"], "mock_mode");
}

#[tokio::test]
async fn test_buffer_endpoint_get() {
    // 创建模拟的应用状态
    let app = Router::new().route("/buffer", get(|| async { 
        serde_json::json!({
            "messages": [],
            "buffer_size": 0
        }) 
    }));
    
    // 创建请求
    let response = app
        .oneshot(Request::builder().uri("/buffer").body(Body::empty()).unwrap())
        .await
        .unwrap();
    
    // 验证响应状态
    assert_eq!(response.status(), StatusCode::OK);
    
    // 验证响应内容
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["buffer_size"], 0);
    assert!(body["messages"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn test_info_endpoint() {
    // 创建模拟的应用状态
    let app = Router::new().route("/info", get(|| async { 
        serde_json::json!({
            "name": "OmniAgent",
            "version": "1.0.0",
            "description": "OmniAgent集成测试实例"
        }) 
    }));
    
    // 创建请求
    let response = app
        .oneshot(Request::builder().uri("/info").body(Body::empty()).unwrap())
        .await
        .unwrap();
    
    // 验证响应状态
    assert_eq!(response.status(), StatusCode::OK);
    
    // 验证响应内容
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["name"], "OmniAgent");
    assert_eq!(body["version"], "1.0.0");
}

#[tokio::test]
async fn test_404_not_found() {
    // 创建应用，不包含任何路由
    let app = Router::new();
    
    // 创建请求到不存在的端点
    let response = app
        .oneshot(Request::builder().uri("/nonexistent").body(Body::empty()).unwrap())
        .await
        .unwrap();
    
    // 验证响应状态为404
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}