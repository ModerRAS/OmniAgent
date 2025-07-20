use omni_agent::{AppConfig, OmniApp};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("🚀 Starting OmniAgent Application...");

    // Load configuration
    let mut config = AppConfig::load_from_env()?;
    config.override_with_env();

    info!("📋 Configuration loaded:");
    info!("  LLM Provider: {}", config.llm.provider);
    info!("  MCP Servers: {}", config.mcp.servers.len());
    info!("  A2A Servers: {}", config.a2a.servers.len());
    info!("  Server Port: {}", config.server.port);

    // Create and run application
    let app = OmniApp::new(config).await?;
    app.run().await?;

    Ok(())
}
