//! MCP协议集成测试

use omni_agent::mcp::{MCPClient, MCPMessage};
use omni_agent::protocol::message::{Message, MessageRole};
use tokio;

// Mock MCP服务器地址
const MOCK_MCP_SERVER: &str = "http://127.0.0.1:8081";

#[tokio::test]
async fn test_mcp_client_creation() {
    // 测试MCP客户端创建
    let client = MCPClient::new(MOCK_MCP_SERVER.to_string());
    
    // 验证客户端成功创建
    assert_eq!(client.base_url(), MOCK_MCP_SERVER);
}

#[tokio::test]
async fn test_mcp_message_struct() {
    // 测试MCP消息结构体
    let message = MCPMessage {
        method: "test.method".to_string(),
        params: serde_json::json!({"test": "parameter"}),
        id: Some(123),
    };
    
    // 验证消息字段
    assert_eq!(message.method, "test.method");
    assert_eq!(message.params["test"], "parameter");
    assert_eq!(message.id, Some(123));
}

#[tokio::test]
async fn test_mcp_protocol_message_conversion() {
    // 测试MCP协议消息转换
    let message = Message {
        role: MessageRole::System,
        content: "测试MCP协议消息".to_string(),
    };
    
    // 验证消息转换字段
    assert_eq!(message.role, MessageRole::System);
    assert_eq!(message.content, "测试MCP协议消息");
}

#[tokio::test]
async fn test_multiple_mcp_clients() {
    // 测试创建多个MCP客户端
    let client1 = MCPClient::new("http://server1:8081".to_string());
    let client2 = MCPClient::new("http://server2:8081".to_string());
    
    // 验证客户端地址不同
    assert_eq!(client1.base_url(), "http://server1:8081");
    assert_eq!(client2.base_url(), "http://server2:8081");
}

#[tokio::test]
async fn test_mcp_client_with_different_ports() {
    // 测试不同端口的MCP客户端
    let ports = vec![8081, 8083, 8085];
    
    for port in ports {
        let url = format!("http://127.0.0.1:{}", port);
        let client = MCPClient::new(url.clone());
        assert_eq!(client.base_url(), url);
    }
}

#[tokio::test]
async fn test_mcp_message_with_null_id() {
    // 测试MCP消息带空ID
    let message = MCPMessage {
        method: "test.method.null".to_string(),
        params: serde_json::json!({}),
        id: None,
    };
    
    // 验证空ID处理
    assert_eq!(message.method, "test.method.null");
    assert_eq!(message.id, None);
}