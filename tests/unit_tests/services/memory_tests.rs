//! 内存服务单元测试

use omni_agent::services::memory::{MemoryService, ContextData, CompressedContext};
use tokio;

#[tokio::test]
async fn test_compress_context() {
    let memory_service = MemoryService::new();
    
    let context = ContextData {
        messages: vec![
            "用户: 你好".to_string(),
            "助手: 你好！有什么可以帮助你的吗？".to_string(),
        ],
    };
    
    let compressed = memory_service.compress_context(context).await;
    
    // 验证压缩结果
    assert_eq!(compressed.original_token_count, 6);
    assert_eq!(compressed.compressed_token_count, 6);
    assert!(!compressed.summary.is_empty());
    assert!(compressed.summary.contains("用户: 你好"));
    assert!(compressed.summary.contains("助手: 你好！有什么可以帮助你的吗？"));
}

#[tokio::test]
async fn test_compress_empty_context() {
    let memory_service = MemoryService::new();
    
    let context = ContextData {
        messages: vec![],
    };
    
    let compressed = memory_service.compress_context(context).await;
    
    // 验证空上下文的压缩结果
    assert_eq!(compressed.original_token_count, 0);
    assert_eq!(compressed.compressed_token_count, 0);
    assert_eq!(compressed.summary, "");
}

#[tokio::test]
async fn test_compress_large_context() {
    let memory_service = MemoryService::new();
    
    // 创建一个较大的上下文
    let mut messages = vec![];
    for i in 0..10 {
        messages.push(format!("用户消息 {}", i));
        messages.push(format!("助手回复 {}", i));
    }
    
    let context = ContextData { messages };
    let compressed = memory_service.compress_context(context).await;
    
    // 验证大上下文的压缩结果
    assert_eq!(compressed.original_token_count, 20);
    assert_eq!(compressed.compressed_token_count, 20);
    assert!(!compressed.summary.is_empty());
    assert!(compressed.summary.len() > 0);
}

#[tokio::test]
async fn test_compressed_context_fields() {
    let memory_service = MemoryService::new();
    
    let context = ContextData {
        messages: vec![
            "测试消息1".to_string(),
            "测试消息2".to_string(),
        ],
    };
    
    let compressed = memory_service.compress_context(context).await;
    
    // 验证CompressedContext结构体的所有字段
    assert_eq!(compressed.original_token_count, 4);
    assert_eq!(compressed.compressed_token_count, 4);
    assert!(!compressed.summary.is_empty());
    
    // 验证压缩后的令牌数不超过原始令牌数
    assert!(compressed.compressed_token_count <= compressed.original_token_count);
}

#[tokio::test]
async fn test_context_data_fields() {
    let messages = vec![
        "消息1".to_string(),
        "消息2".to_string(),
        "消息3".to_string(),
    ];
    
    let context = ContextData { messages };
    
    // 验证ContextData结构体的字段
    assert_eq!(context.messages.len(), 3);
    assert_eq!(context.messages[0], "消息1");
    assert_eq!(context.messages[1], "消息2");
    assert_eq!(context.messages[2], "消息3");
}