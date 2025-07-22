use crate::mock_servers::*;
use omni_agent::{
    llm::providers::{ClaudeConfig, ProviderConfig},
    llm::LLMConfig,
    AgentBuilder,
};

mod mock_servers;

#[tokio::test]
#[ignore = "Integration test requiring mock server"]
async fn test_agent_with_claude_llm() {
    // 启动mock服务器
    let _mock_claude_url = start_mock_claude_server(8083).await;

    // 配置Claude provider
    let provider_config = ProviderConfig {
        openai: None,
        claude: Some(ClaudeConfig {
            api_key: "test-key".to_string(),
            model: "claude-3-haiku-20240307".to_string(),
            base_url: None,
        }),
        google: None,
    };

    let llm_config = LLMConfig {
        provider: "claude".to_string(),
        model: "claude-3-haiku-20240307".to_string(),
        temperature: 0.7,
        max_tokens: 150,
        use_mock: false,
    };

    // 创建agent
    let agent = AgentBuilder::new("test-agent", "Test agent")
        .build()
        .await
        .unwrap();

    // 配置LLM使用mock服务器
    {
        let mut llm_service = agent.llm.write().await;
        llm_service.config = llm_config;
        llm_service.manager = omni_agent::llm::manager::LLMManager::new(provider_config, "claude");

        // 修改Claude provider的base_url指向mock服务器 - 这里简化处理，通过ProviderConfig设置
        // 实际测试中可以直接在ProviderConfig中设置base_url
    }

    // 测试消息处理
    let response = agent
        .llm
        .write()
        .await
        .process_message("Can you help me with a simple calculation?", &[])
        .await
        .unwrap();

    if let omni_agent::protocol::message::MessageContent::Text { text } = &response.content {
        assert!(!text.is_empty());
    } else {
        panic!("Expected text content");
    }
    println!("Agent response: {:?}", response.content);

    // 测试mock模式
    {
        let mut llm_service = agent.llm.write().await;
        llm_service.config.use_mock = true;
    }

    let mock_response = agent
        .llm
        .write()
        .await
        .process_message("Test mock mode", &[])
        .await
        .unwrap();

    if let omni_agent::protocol::message::MessageContent::Text { text } = &mock_response.content {
        assert!(text.contains("[Claude"));
    } else {
        panic!("Expected text content");
    }
    println!("Mock mode response: {:?}", mock_response.content);
}

#[tokio::test]
async fn test_agent_capabilities() {
    let agent = AgentBuilder::new("capability-test", "Test capabilities")
        .build()
        .await
        .unwrap();

    // 测试默认配置
    assert_eq!(agent.config.name, "capability-test");
    assert_eq!(agent.config.description, "Test capabilities");

    // 测试初始状态
    let providers = agent.llm.write().await.list_providers().await;
    println!("Initial providers: {providers:?}");

    // 测试mock响应
    let response = agent
        .llm
        .write()
        .await
        .process_message("Hello", &[])
        .await
        .unwrap();

    if let omni_agent::protocol::message::MessageContent::Text { text } = &response.content {
        assert!(!text.is_empty());
    } else {
        panic!("Expected text content");
    }
    println!("Default mock response: {:?}", response.content);
}
