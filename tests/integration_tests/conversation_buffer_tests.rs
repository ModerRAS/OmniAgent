//! 对话缓冲区专门测试

use omni_agent::core::state::{StateManager, BufferedMessage, MessageType};
use tokio;
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_conversation_buffer_capacity_management() {
    let state_manager = StateManager::new();
    
    // 添加超过默认容量的消息
    let capacity = 10; // 默认缓冲区容量
    for i in 0..(capacity + 5) {
        let message = BufferedMessage {
            id: Uuid::new_v4(),
            content: format!("消息 {}", i),
            timestamp: Utc::now(),
            message_type: MessageType::UserMessage,
            context_relevance: 0.8,
        };
        state_manager.add_to_buffer(message).await.unwrap();
    }
    
    // 验证缓冲区大小保持在容量限制内
    assert_eq!(state_manager.buffer_size(), capacity);
    
    let messages = state_manager.get_buffer_messages().await;
    assert_eq!(messages.len(), capacity);
    
    // 验证最旧的消息被移除，最新的消息保留
    assert_eq!(messages[0].content, "消息 5"); // 最旧的保留消息
    assert_eq!(messages[capacity-1].content, "消息 14"); // 最新的消息
}

#[tokio::test]
async fn test_conversation_buffer_message_order() {
    let state_manager = StateManager::new();
    
    // 按顺序添加消息
    let messages_content = vec!["第一", "第二", "第三", "第四", "第五"];
    for content in &messages_content {
        let message = BufferedMessage {
            id: Uuid::new_v4(),
            content: content.to_string(),
            timestamp: Utc::now(),
            message_type: MessageType::UserMessage,
            context_relevance: 0.8,
        };
        state_manager.add_to_buffer(message).await.unwrap();
    }
    
    // 验证消息顺序保持不变
    let messages = state_manager.get_buffer_messages().await;
    assert_eq!(messages.len(), 5);
    
    for (i, message) in messages.iter().enumerate() {
        assert_eq!(message.content, messages_content[i]);
    }
}

#[tokio::test]
async fn test_conversation_buffer_clear_and_reset() {
    let state_manager = StateManager::new();
    
    // 添加一些消息
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
    
    // 验证消息已添加
    assert_eq!(state_manager.buffer_size(), 3);
    
    // 清空缓冲区
    state_manager.clear_buffer().await.unwrap();
    
    // 验证缓冲区已清空
    assert_eq!(state_manager.buffer_size(), 0);
    
    // 再次添加消息
    let message = BufferedMessage {
        id: Uuid::new_v4(),
        content: "新的消息".to_string(),
        timestamp: Utc::now(),
        message_type: MessageType::UserMessage,
        context_relevance: 0.8,
    };
    state_manager.add_to_buffer(message).await.unwrap();
    
    // 验证缓冲区可以正常使用
    assert_eq!(state_manager.buffer_size(), 1);
    let messages = state_manager.get_buffer_messages().await;
    assert_eq!(messages[0].content, "新的消息");
}

#[tokio::test]
async fn test_conversation_buffer_message_types() {
    let state_manager = StateManager::new();
    
    // 添加不同类型的消息
    let messages = vec![
        BufferedMessage {
            id: Uuid::new_v4(),
            content: "用户消息".to_string(),
            timestamp: Utc::now(),
            message_type: MessageType::UserMessage,
            context_relevance: 0.9,
        },
        BufferedMessage {
            id: Uuid::new_v4(),
            content: "系统消息".to_string(),
            timestamp: Utc::now(),
            message_type: MessageType::SystemMessage,
            context_relevance: 0.7,
        },
        BufferedMessage {
            id: Uuid::new_v4(),
            content: "工具响应".to_string(),
            timestamp: Utc::now(),
            message_type: MessageType::ToolResponse,
            context_relevance: 0.8,
        },
        BufferedMessage {
            id: Uuid::new_v4(),
            content: "LLM响应".to_string(),
            timestamp: Utc::now(),
            message_type: MessageType::LLMResponse,
            context_relevance: 0.85,
        },
    ];
    
    // 添加所有消息
    for message in &messages {
        state_manager.add_to_buffer(message.clone()).await.unwrap();
    }
    
    // 验证所有消息都被正确存储
    let stored_messages = state_manager.get_buffer_messages().await;
    assert_eq!(stored_messages.len(), 4);
    
    // 验证消息类型保持正确
    assert_eq!(stored_messages[0].message_type, MessageType::UserMessage);
    assert_eq!(stored_messages[1].message_type, MessageType::SystemMessage);
    assert_eq!(stored_messages[2].message_type, MessageType::ToolResponse);
    assert_eq!(stored_messages[3].message_type, MessageType::LLMResponse);
    
    // 验证内容保持正确
    assert_eq!(stored_messages[0].content, "用户消息");
    assert_eq!(stored_messages[1].content, "系统消息");
    assert_eq!(stored_messages[2].content, "工具响应");
    assert_eq!(stored_messages[3].content, "LLM响应");
}

#[tokio::test]
async fn test_conversation_buffer_concurrent_access() {
    let state_manager = StateManager::new();
    let state_manager_clone = state_manager.clone();
    
    // 并发添加消息
    let mut handles = vec![];
    
    for i in 0..10 {
        let manager = state_manager.clone();
        let handle = tokio::spawn(async move {
            let message = BufferedMessage {
                id: Uuid::new_v4(),
                content: format!("并发消息 {}", i),
                timestamp: Utc::now(),
                message_type: MessageType::UserMessage,
                context_relevance: 0.8,
            };
            manager.add_to_buffer(message).await
        });
        handles.push(handle);
    }
    
    // 等待所有操作完成
    let results: Vec<_> = futures::future::join_all(handles).await;
    
    // 验证所有操作都成功
    for result in results {
        assert!(result.is_ok());
        assert!(result.unwrap().is_ok());
    }
    
    // 验证最终状态
    assert_eq!(state_manager_clone.buffer_size(), 10);
    let messages = state_manager_clone.get_buffer_messages().await;
    assert_eq!(messages.len(), 10);
}