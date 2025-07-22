use crate::mock_servers::*;
use omni_agent::{
    llm::providers::claude::ClaudeProvider,
    llm::providers::{LLMProvider, LLMRequest, Message, MessageRole},
};

mod mock_servers;

#[tokio::test]
#[ignore = "Integration test requiring mock server"]
async fn test_claude_provider_with_mock_server() {
    // 启动模拟Claude服务器
    let mock_url = start_mock_claude_server(8081).await;

    // 创建Claude provider并指向模拟服务器
    let mut provider = ClaudeProvider::new(
        "mock-key".to_string(),
        Some("claude-3-haiku-20240307".to_string()),
        None,
    );
    provider.base_url = mock_url;

    // 创建测试请求
    let request = LLMRequest {
        messages: vec![Message {
            role: MessageRole::User,
            content: "Hello".to_string(),
        }],
        model: "claude-3-haiku-20240307".to_string(),
        temperature: Some(0.7),
        max_tokens: Some(50),
        stream: Some(false),
    };

    // 发送请求
    let response = provider.chat(request).await.unwrap();

    // 验证响应
    assert!(!response.content.is_empty());
    assert!(response.content.contains("mock"));
    assert_eq!(response.model, "claude-3-haiku-20240307");
    assert!(response.usage.is_some());

    println!("Mock Claude response: {}", response.content);
}

#[tokio::test]
#[ignore = "Integration test requiring mock server"]
async fn test_claude_provider_with_system_message() {
    let mock_url = start_mock_claude_server(8082).await;

    let mut provider = ClaudeProvider::new(
        "mock-key".to_string(),
        Some("claude-3-sonnet-20240229".to_string()),
        None,
    );
    provider.base_url = mock_url;

    let request = LLMRequest {
        messages: vec![
            Message {
                role: MessageRole::System,
                content: "You are a helpful assistant".to_string(),
            },
            Message {
                role: MessageRole::User,
                content: "What is 2+2?".to_string(),
            },
        ],
        model: "claude-3-sonnet-20240229".to_string(),
        temperature: Some(0.5),
        max_tokens: Some(100),
        stream: Some(false),
    };

    let response = provider.chat(request).await.unwrap();
    assert!(!response.content.is_empty());
    println!("System message test response: {}", response.content);
}

#[tokio::test]
async fn test_claude_provider_error_handling() {
    // 使用无效的URL测试错误处理
    let _provider = ClaudeProvider::new(
        "invalid-key".to_string(),
        Some("claude-3-haiku-20240307".to_string()),
        None,
    );
    let _request = LLMRequest {
        messages: vec![Message {
            role: MessageRole::User,
            content: "Test".to_string(),
        }],
        model: "claude-3-haiku-20240307".to_string(),
        temperature: Some(0.7),
        max_tokens: Some(50),
        stream: Some(false),
    };
}
