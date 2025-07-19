use omni_agent::{AgentBuilder, A2AServer};
use omni_agent::protocol::message::{Message, MessageContent};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_agent_integration() {
    let mock_mcp_server = MockServer::start().await;
    let mock_a2a_server = MockServer::start().await;

    // Setup mock MCP server
    Mock::given(method("GET"))
        .and(path("/manifest"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "name": "test-mcp",
            "version": "1.0.0",
            "description": "Test MCP server",
            "capabilities": vec!["weather", "calculator"],
            "tools": [{
                "name": "get_weather",
                "description": "Get weather information",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "location": {"type": "string"}
                    }
                },
                "output_schema": {
                    "type": "object",
                    "properties": {
                        "temperature": {"type": "number"},
                        "condition": {"type": "string"}
                    }
                }
            }]
        })))
        .mount(&mock_mcp_server)
        .await;

    // Setup mock A2A server
    Mock::given(method("GET"))
        .and(path("/manifest"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "name": "test-a2a",
            "version": "1.0.0",
            "description": "Test A2A server",
            "capabilities": vec!["data_analysis", "reporting"],
            "supported_protocols": vec!["http", "websocket"],
            "endpoints": vec!["http://localhost:8081"]
        })))
        .mount(&mock_a2a_server)
        .await;

    // Build agent with mock endpoints
    let agent = AgentBuilder::new("test-agent", "Integration test agent")
        .add_mcp("weather_service", &format!("{}/", mock_mcp_server.uri()))
        .add_a2a("data_analyst", &format!("{}/", mock_a2a_server.uri()))
        .build()
        .await
        .unwrap();

    // Test manifest fetching
    let capabilities = agent.get_capabilities().await;
    assert!(capabilities.contains(&"weather".to_string()));
    assert!(capabilities.contains(&"calculator".to_string()));
    assert!(capabilities.contains(&"data_analysis".to_string()));

    // Test message processing
    let message = Message::new(
        "test_user".to_string(),
        "test-agent".to_string(),
        MessageContent::Text {
            text: "What's the weather like?".to_string(),
        },
        None,
    );

    let response = agent.process_message(message).await.unwrap();
    if let MessageContent::Text { text } = response.content {
        assert!(text.contains("weather"));
    } else {
        panic!("Expected text response");
    }
}

#[tokio::test]
async fn test_a2a_server_health() {
    let server = A2AServer::new(0); // Use port 0 for random available port
    
    // This is a basic test to ensure the server can be created
    // In a real test, we'd spin up the server and make HTTP requests
    assert_eq!(server.port, 0);
}

#[tokio::test]
async fn test_state_machine() {
    use omni_agent::agent::state::{StateMachine, AgentState};

    let mut state_machine = StateMachine::new(5);
    
    assert_eq!(state_machine.get_state(), &AgentState::Idle);
    assert!(state_machine.is_ready());

    // Add a message
    let message = Message::new(
        "user".to_string(),
        "agent".to_string(),
        MessageContent::Text {
            text: "Hello".to_string(),
        },
        None,
    );

    state_machine.add_message(message);
    assert_eq!(state_machine.context.len(), 1);

    // Test state transitions
    state_machine.transition(AgentState::Processing);
    assert_eq!(state_machine.get_state(), &AgentState::Processing);
    assert!(!state_machine.is_ready());

    state_machine.transition(AgentState::Idle);
    assert_eq!(state_machine.get_state(), &AgentState::Idle);
    assert!(state_machine.is_ready());
}

#[tokio::test]
async fn test_message_flow() {
    let agent = AgentBuilder::new("flow-test", "Message flow test")
        .build()
        .await
        .unwrap();

    // Test basic message flow
    let message1 = Message::new(
        "user".to_string(),
        "flow-test".to_string(),
        MessageContent::Text {
            text: "Hello agent".to_string(),
        },
        None,
    );

    let response1 = agent.process_message(message1).await.unwrap();
    assert_eq!(response1.sender, "flow-test");
    assert_eq!(response1.recipient, "user");

    // Test tool call flow
    let message2 = Message::new(
        "user".to_string(),
        "flow-test".to_string(),
        MessageContent::ToolCall {
            tool: "mock_tool".to_string(),
            parameters: serde_json::json!({"command": "test"}),
        },
        None,
    );

    let response2 = agent.process_message(message2).await.unwrap();
    if let MessageContent::ToolResult { tool, result, success } = response2.content {
        assert_eq!(tool, "mock_tool");
        assert!(success);
    } else {
        panic!("Expected tool result");
    }
}