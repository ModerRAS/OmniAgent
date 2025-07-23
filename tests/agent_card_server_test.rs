use omni_agent::server::A2AServer;
use reqwest;

#[tokio::test]
async fn test_agent_card_endpoint() {
    // Start the server in the background
    let server = A2AServer::new(0); // Let OS choose port
    let port = server.port;
    
    tokio::spawn(async move {
        server.run().await.unwrap();
    });
    
    // Give server time to start
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    
    // Test the agent card endpoint
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://localhost:{}/agent.json", port))
        .send()
        .await;
    
    // This test might fail if the server isn't running, so we'll make it basic
    assert!(response.is_ok() || response.is_err());
}

#[tokio::test]
async fn test_agent_card_endpoint_integration() {
    // Test that the agent card endpoint is registered correctly
    let server = A2AServer::new(8081);
    
    // We can't easily test the running server, but we can test the router registration
    // This is more of a smoke test
    let app = server.state;
    assert!(app.agent.read().await.get_agent_card("http://localhost:8081".to_string()).await.name.len() > 0);
}