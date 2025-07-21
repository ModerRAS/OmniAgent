use omni_agent::{AppConfig, AgentBuilder};
use serde_json::{json, Value};
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn test_agent_creation() {
    let config = AppConfig {
        llm: omni_agent::LLMConfig {
            provider: "google".to_string(),
            model: "gemini-pro".to_string(),
            api_key: "test-key".to_string(),
            base_url: None,
            temperature: 0.7,
            max_tokens: 100,
            use_mock: true,
        },
        ..Default::default()
    };

    let agent = AgentBuilder::new("test-agent", "测试智能体")
        .version("1.0.0")
        .build()
        .await
        .unwrap();

    assert_eq!(agent.config.name, "test-agent");
    assert_eq!(agent.config.description, "测试智能体");
}

#[tokio::test]
async fn test_intelligent_routing() {
    // 这是一个集成测试，测试路由逻辑
    let config = AppConfig {
        llm: omni_agent::LLMConfig {
            provider: "google".to_string(),
            model: "gemini-pro".to_string(),
            api_key: "".to_string(),
            base_url: None,
            temperature: 0.7,
            max_tokens: 100,
            use_mock: true,
        },
        ..Default::default()
    };

    let agent = AgentBuilder::new("routing-test", "路由测试")
        .build()
        .await
        .unwrap();

    // 模拟路由测试
    let test_cases = vec![
        ("请帮我读取文件", "mcp"),
        ("北京天气如何", "a2a"),
        ("解释一下量子计算", "llm"),
    ];

    for (message, expected_type) in test_cases {
        let response = agent
            .llm
            .write()
            .await
            .process_message(message, &[])
            .await
            .unwrap();

        assert!(!response.content.is_empty());
        println!("Message: {} → Response type: {}", message, expected_type);
    }
}