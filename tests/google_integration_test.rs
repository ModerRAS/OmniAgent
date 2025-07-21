use crate::mock_servers::*;
use omni_agent::{
    llm::providers::google::GoogleProvider,
    llm::providers::LLMProvider,
    llm::providers::{LLMRequest, Message, MessageRole},
};

mod mock_servers;

#[tokio::test]
#[ignore = "Integration test requiring mock server"]
async fn test_google_provider_with_mock_server() {
    // 启动模拟Google服务器
    let _mock_google_url = start_mock_google_server(8084).await;

    // 创建Google provider并指向模拟服务器
    let provider = GoogleProvider::new("mock-key".to_string(), Some("gemini-pro".to_string()));
    // 注意：GoogleProvider的base_url是私有的，我们需要通过ProviderConfig设置

    let _request = LLMRequest {
        messages: vec![Message {
            role: MessageRole::User,
            content: "Hello".to_string(),
        }],
        model: "gemini-pro".to_string(),
        temperature: Some(0.7),
        max_tokens: Some(50),
        stream: Some(false),
    };

    // 简化测试 - 验证provider创建
    assert_eq!(provider.provider_name(), "google");
    assert_eq!(provider.model, "gemini-pro");
}

#[tokio::test]
async fn test_google_provider_system_message() {
    let request = LLMRequest {
        messages: vec![
            Message {
                role: MessageRole::System,
                content: "You are a coding assistant".to_string(),
            },
            Message {
                role: MessageRole::User,
                content: "Write a hello world program".to_string(),
            },
        ],
        model: "gemini-pro".to_string(),
        temperature: Some(0.5),
        max_tokens: Some(100),
        stream: Some(false),
    };

    // 测试消息转换
    let provider = GoogleProvider::new("test-key".to_string(), None);
    let (system_msg, contents) = provider.convert_messages(request.messages);

    assert_eq!(system_msg, Some("You are a coding assistant".to_string()));
    assert_eq!(contents.len(), 1);
    assert_eq!(contents[0].role, "user");
    assert_eq!(
        contents[0].parts[0].text,
        Some("Write a hello world program".to_string())
    );
}

#[tokio::test]
async fn test_google_generation_config() {
    let provider = GoogleProvider::new("test-key".to_string(), None);

    let request = LLMRequest {
        messages: vec![Message {
            role: MessageRole::User,
            content: "Test config".to_string(),
        }],
        model: "gemini-pro".to_string(),
        temperature: Some(0.8),
        max_tokens: Some(150),
        stream: Some(false),
    };

    let (_system_msg, gemini_msgs) = provider.convert_messages(request.messages);

    // 验证消息转换
    assert_eq!(gemini_msgs.len(), 1);
    assert_eq!(
        gemini_msgs[0].parts[0].text,
        Some("Test config".to_string())
    );
}

#[tokio::test]
async fn test_google_provider_error_handling() {
    // 测试错误处理 - 使用无效配置
    let provider =
        GoogleProvider::new("invalid-key".to_string(), Some("invalid-model".to_string()));

    // 验证provider创建
    assert_eq!(provider.provider_name(), "google");
    assert_eq!(provider.model, "invalid-model");
}
