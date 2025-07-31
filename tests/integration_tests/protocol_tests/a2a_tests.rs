//! A2A协议集成测试

use omni_agent::a2a::{A2AClient, A2AMessage};
use omni_agent::protocol::message::{Message, MessageRole};
use tokio;

// Mock A2A服务器地址
const MOCK_A2A_SERVER: &str = "http://127.0.0.1:8080";

#[tokio::test]
async fn test_a2a_client_creation() {
    // 测试A2A客户端创建
    let client = A2AClient::new(MOCK_A2A_SERVER.to_string());
    
    // 验证客户端成功创建
    assert_eq!(client.base_url(), MOCK_A2A_SERVER);
}

#[tokio::test]
async fn test_a2a_message_struct() {
    // 测试A2A消息结构体
    let message = A2AMessage {
        id: "test-message-1".to_string(),
        content: "测试A2A消息内容".to_string(),
        sender: "test-sender".to_string(),
        recipient: "test-recipient".to_string(),
        timestamp: chrono::Utc::now(),
    };
    
    // 验证消息字段
    assert_eq!(message.id, "test-message-1");
    assert_eq!(message.content, "测试A2A消息内容");
    assert_eq!(message.sender, "test-sender");
    assert_eq!(message.recipient, "test-recipient");
    assert!(message.timestamp.timestamp() > 0);
}

#[tokio::test]
async fn test_a2a_protocol_message_conversion() {
    // 测试A2A协议消息转换
    let message = Message {
        role: MessageRole::User,
        content: "测试协议消息".to_string(),
    };
    
    // 验证消息转换字段
    assert_eq!(message.role, MessageRole::User);
    assert_eq!(message.content, "测试协议消息");
}

#[tokio::test]
async fn test_multiple_a2a_clients() {
    // 测试创建多个A2A客户端
    let client1 = A2AClient::new("http://server1:8080".to_string());
    let client2 = A2AClient::new("http://server2:8080".to_string());
    
    // 验证客户端地址不同
    assert_eq!(client1.base_url(), "http://server1:8080");
    assert_eq!(client2.base_url(), "http://server2:8080");
}

#[tokio::test]
async fn test_a2a_client_with_different_ports() {
    // 测试不同端口的A2A客户端
    let ports = vec![8080, 8081, 8082];
    
    for port in ports {
        let url = format!("http://127.0.0.1:{}", port);
        let client = A2AClient::new(url.clone());
        assert_eq!(client.base_url(), url);
    }
}