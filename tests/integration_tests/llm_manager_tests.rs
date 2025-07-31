//! LLM管理器集成测试

use omni_agent::llm::{LLMManager, ProviderConfig, LLMRequest, Message, MessageRole};
use std::collections::HashMap;
use tokio;

#[tokio::test]
async fn test_llm_manager_creation() {
    // 创建提供者配置
    let mut providers = HashMap::new();
    providers.insert("mock".to_string(), ProviderConfig {
        api_key: "mock-key".to_string(),
        model: Some("mock-model".to_string()),
        base_url: None,
    });
    
    // 创建LLM管理器
    let llm_manager = LLMManager::new(providers, true).await;
    
    // 验证LLM管理器成功创建
    assert!(llm_manager.is_ok());
}

#[tokio::test]
async fn test_llm_manager_with_multiple_providers() {
    // 创建多个提供者配置
    let mut providers = HashMap::new();
    providers.insert("mock1".to_string(), ProviderConfig {
        api_key: "mock-key-1".to_string(),
        model: Some("mock-model-1".to_string()),
        base_url: None,
    });
    providers.insert("mock2".to_string(), ProviderConfig {
        api_key: "mock-key-2".to_string(),
        model: Some("mock-model-2".to_string()),
        base_url: None,
    });
    
    // 创建LLM管理器
    let llm_manager = LLMManager::new(providers, true).await;
    
    // 验证LLM管理器成功创建
    assert!(llm_manager.is_ok());
}

#[tokio::test]
async fn test_llm_manager_chat_request() {
    // 创建提供者配置
    let mut providers = HashMap::new();
    providers.insert("mock".to_string(), ProviderConfig {
        api_key: "mock-key".to_string(),
        model: Some("mock-model".to_string()),
        base_url: None,
    });
    
    // 创建LLM管理器
    let llm_manager = LLMManager::new(providers, true).await.unwrap();
    
    // 创建聊天请求
    let request = LLMRequest {
        messages: vec![
            Message {
                role: MessageRole::User,
                content: "你好，这是一个测试".to_string(),
            }
        ],
        model: "mock-model".to_string(),
        temperature: Some(0.7),
        max_tokens: Some(100),
        stream: Some(false),
    };
    
    // 发送请求
    let response = llm_manager.chat("mock", request).await;
    
    // 验证响应
    assert!(response.is_ok());
    let response = response.unwrap();
    assert!(response.content.contains("模拟响应"));
    assert!(response.usage.is_some());
}

#[tokio::test]
async fn test_llm_manager_provider_switching() {
    // 创建提供者配置
    let mut providers = HashMap::new();
    providers.insert("mock1".to_string(), ProviderConfig {
        api_key: "mock-key-1".to_string(),
        model: Some("mock-model-1".to_string()),
        base_url: None,
    });
    providers.insert("mock2".to_string(), ProviderConfig {
        api_key: "mock-key-2".to_string(),
        model: Some("mock-model-2".to_string()),
        base_url: None,
    });
    
    // 创建LLM管理器
    let llm_manager = LLMManager::new(providers, true).await.unwrap();
    
    // 测试使用第一个提供者
    let request1 = LLMRequest {
        messages: vec![Message {
            role: MessageRole::User,
            content: "测试提供者1".to_string(),
        }],
        model: "mock-model-1".to_string(),
        temperature: None,
        max_tokens: None,
        stream: None,
    };
    
    let response1 = llm_manager.chat("mock1", request1).await;
    assert!(response1.is_ok());
    
    // 测试使用第二个提供者
    let request2 = LLMRequest {
        messages: vec![Message {
            role: MessageRole::User,
            content: "测试提供者2".to_string(),
        }],
        model: "mock-model-2".to_string(),
        temperature: None,
        max_tokens: None,
        stream: None,
    };
    
    let response2 = llm_manager.chat("mock2", request2).await;
    assert!(response2.is_ok());
}

#[tokio::test]
async fn test_llm_manager_with_empty_providers() {
    // 创建空的提供者配置
    let providers = HashMap::new();
    
    // 创建LLM管理器
    let llm_manager = LLMManager::new(providers, true).await;
    
    // 验证在模拟模式下即使没有提供者也能创建
    assert!(llm_manager.is_ok());
}