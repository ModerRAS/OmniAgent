//! 状态管理器单元测试

use omni_agent::core::state::{StateManager, BufferedMessage, MessageType};
use tokio;
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_add_message_to_buffer() {
    let state_manager = StateManager::new();
    let message = BufferedMessage {
        id: Uuid::new_v4(),
        content: "测试消息".to_string(),
        timestamp: Utc::now(),
        message_type: MessageType::UserMessage,
        context_relevance: 0.8,
    };
    
    let result = state_manager.add_to_buffer(message).await;
    assert!(result.is_ok());
    
    let messages = state_manager.get_buffer_messages().await;
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].content, "测试消息");
}

#[tokio::test]
async fn test_get_buffer_messages() {
    let state_manager = StateManager::new();
    
    // 添加多条消息
    for i in 0..3 {
        let message = BufferedMessage {
            id: Uuid::new_v4(),
            content: format!("测试消息 {}", i),
            timestamp: Utc::now(),
            message_type: MessageType::UserMessage,
            context_relevance: 0.8,
        };
        state_manager.add_to_buffer(message).await.unwrap();
    }
    
    let messages = state_manager.get_buffer_messages().await;
    assert_eq!(messages.len(), 3);
    
    // 验证消息顺序
    assert_eq!(messages[0].content, "测试消息 0");
    assert_eq!(messages[1].content, "测试消息 1");
    assert_eq!(messages[2].content, "测试消息 2");
}

#[tokio::test]
async fn test_buffer_size_limit() {
    let state_manager = StateManager::new();
    
    // 添加超过缓冲区大小的消息
    for i in 0..15 {
        let message = BufferedMessage {
            id: Uuid::new_v4(),
            content: format!("测试消息 {}", i),
            timestamp: Utc::now(),
            message_type: MessageType::UserMessage,
            context_relevance: 0.8,
        };
        state_manager.add_to_buffer(message).await.unwrap();
    }
    
    // 缓冲区大小应该保持在限制范围内
    let buffer_size = state_manager.buffer_size();
    assert_eq!(buffer_size, 10); // 默认缓冲区大小为10
    
    let messages = state_manager.get_buffer_messages().await;
    assert_eq!(messages.len(), 10);
    
    // 验证最新的消息在缓冲区中
    assert_eq!(messages[9].content, "测试消息 14");
}

#[tokio::test]
async fn test_clear_buffer() {
    let state_manager = StateManager::new();
    
    // 添加一些消息
    for i in 0..5 {
        let message = BufferedMessage {
            id: Uuid::new_v4(),
            content: format!("测试消息 {}", i),
            timestamp: Utc::now(),
            message_type: MessageType::UserMessage,
            context_relevance: 0.8,
        };
        state_manager.add_to_buffer(message).await.unwrap();
    }
    
    // 验证消息已添加
    assert_eq!(state_manager.buffer_size(), 5);
    
    // 清空缓冲区
    let result = state_manager.clear_buffer().await;
    assert!(result.is_ok());
    
    // 验证缓冲区已清空
    assert_eq!(state_manager.buffer_size(), 0);
    let messages = state_manager.get_buffer_messages().await;
    assert_eq!(messages.len(), 0);
}

#[tokio::test]
async fn test_buffered_message_fields() {
    let state_manager = StateManager::new();
    let timestamp = Utc::now();
    
    let message = BufferedMessage {
        id: Uuid::new_v4(),
        content: "完整测试消息".to_string(),
        timestamp,
        message_type: MessageType::LLMResponse,
        context_relevance: 0.9,
    };
    
    state_manager.add_to_buffer(message.clone()).await.unwrap();
    let messages = state_manager.get_buffer_messages().await;
    
    assert_eq!(messages.len(), 1);
    let buffered_message = &messages[0];
    
    // 验证所有字段
    assert!(!buffered_message.id.is_nil());
    assert_eq!(buffered_message.content, "完整测试消息");
    assert_eq!(buffered_message.timestamp, timestamp);
    assert_eq!(buffered_message.message_type, MessageType::LLMResponse);
    assert_eq!(buffered_message.context_relevance, 0.9);
}