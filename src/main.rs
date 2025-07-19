mod agent;
mod mcp;
mod a2a;
mod protocol;
mod server;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::server::A2AServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting OmniAgent A2A Server on port {}", port);

    let server = A2AServer::new(port);
    server.run().await?;

    Ok(())
}
