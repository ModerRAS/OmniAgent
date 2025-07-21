use omni_agent::{
    llm::providers::{ClaudeConfig, ProviderConfig},
    llm::LLMConfig,
    AgentBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Claude API Usage Example ===");

    // Example 1: Using mock Claude
    println!("=== Example 1: Mock Claude ===");
    let mut agent = AgentBuilder::new("claude-test-agent", "Test agent with Claude")
        .build()
        .await?;

    let response = agent
        .llm
        .write()
        .await
        .process_message("What is the weather like?", &[])
        .await?;

    println!("Mock Claude Response: {:?}", response.content);

    // Example 2: Using real Claude API (requires API key)
    println!("=== Example 2: Real Claude LLM ===");

    if let Ok(api_key) = std::env::var("ANTHROPIC_API_KEY") {
        let provider_config = ProviderConfig {
            openai: None,
            claude: Some(ClaudeConfig {
                api_key: api_key.clone(),
                model: "claude-3-haiku-20240307".to_string(),
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

        let mut agent = AgentBuilder::new("real-claude-agent", "Agent with Claude API")
            .build()
            .await?;

        // Update the LLM configuration and provider
        {
            let mut llm_service = agent.llm.write().await;
            llm_service.config = llm_config.clone();
            llm_service.manager =
                omni_agent::llm::manager::LLMManager::new(provider_config.clone(), "claude");
        }

        let response = agent
            .llm
            .write()
            .await
            .process_message("Explain quantum computing in one sentence.", &[])
            .await?;

        println!("Real Claude Response: {:?}", response.content);
        println!(
            "Model used: {}",
            response
                .metadata
                .as_ref()
                .and_then(|m| m.get("model"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
        );
    } else {
        println!("Skipping real Claude test - ANTHROPIC_API_KEY not set");
        println!("Set ANTHROPIC_API_KEY environment variable to test real Claude API");
    }

    // Example 3: List available providers
    println!("=== Example 3: Available Providers ===");
    let providers = agent.llm.write().await.list_providers().await;
    println!("Available LLM providers: {:?}", providers);

    Ok(())
}
