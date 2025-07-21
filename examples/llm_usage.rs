use omni_agent::{
    llm::providers::{OpenAIConfig, ProviderConfig},
    llm::LLMConfig,
    AgentBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Using mock LLM
    println!("=== Example 1: Mock LLM ===");
    let mut agent = AgentBuilder::new("test-agent", "Test agent with LLM")
        .build()
        .await?;

    let response = agent
        .llm
        .write()
        .await
        .process_message("What is the weather like?", &[])
        .await?;

    println!("Mock Response: {:?}", response.content);

    // Example 2: Using OpenAI API (requires API key)
    println!("=== Example 2: OpenAI LLM ===");

    // Uncomment and set your API key to use real OpenAI
    // let provider_config = ProviderConfig {
    //     openai: Some(OpenAIConfig {
    //         api_key: std::env::var("OPENAI_API_KEY")?,  // Set your API key
    //         base_url: None,
    //         model: "gpt-3.5-turbo".to_string(),
    //     }),
    //     claude: None,
    //     google: None,
    // };

    // let llm_config = LLMConfig {
    //     provider: "openai".to_string(),
    //     model: "gpt-3.5-turbo".to_string(),
    //     temperature: 0.7,
    //     max_tokens: 150,
    //     use_mock: false,
    // };

    // let agent = AgentBuilder::new("openai-agent", "Agent with OpenAI")
    //     .build()
    //     .await?;

    // agent.llm.write().await.config = llm_config;
    // agent.llm.write().await.manager = LLMManager::new(provider_config, "openai");

    // let response = agent.llm.write().await.process_message(
    //     "What is the capital of France?",
    //     &[]
    // ).await?;
    // println!("OpenAI Response: {:?}", response.content);

    // Example 3: List available providers
    println!("=== Example 3: Available Providers ===");
    let providers = agent.llm.write().await.list_providers().await;
    println!("Available LLM providers: {:?}", providers);

    Ok(())
}
