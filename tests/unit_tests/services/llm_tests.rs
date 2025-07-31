//! LLM服务单元测试

use omni_agent::services::llm::{LLMService, TokenUsage};
use tokio;

#[tokio::test]
async fn test_llm_service_mock_mode() {
    // 使用模拟模式创建LLM服务
    let llm_service = LLMService::new(true);
    
    let (response, token_usage) = llm_service.process_message("你好，世界!", &[]).await.unwrap();
    
    // 验证模拟响应
    assert!(response.contains("模拟LLM响应"));
    assert!(!response.is_empty());
    
    // 验证令牌使用统计
    assert_eq!(token_usage.prompt_tokens, 2);
    assert_eq!(token_usage.completion_tokens, 3);
    assert_eq!(token_usage.total_tokens, 5);
}

#[tokio::test]
async fn test_llm_service_with_context() {
    let llm_service = LLMService::new(true);
    
    let context_messages = vec![
        "用户: 你好".to_string(),
        "助手: 你好！有什么可以帮助你的吗？".to_string(),
    ];
    
    let (response, token_usage) = llm_service.process_message("我很好，谢谢！", &context_messages).await.unwrap();
    
    // 验证响应包含上下文信息
    assert!(response.contains("模拟LLM响应"));
    assert!(!response.is_empty());
    
    // 验证令牌使用统计（应该包含上下文）
    assert_eq!(token_usage.prompt_tokens, 7); // 上下文 + 当前消息
    assert_eq!(token_usage.completion_tokens, 3);
    assert_eq!(token_usage.total_tokens, 10);
}

#[tokio::test]
async fn test_token_calculation() {
    let llm_service = LLMService::new(true);
    
    // 测试不同的消息长度
    let test_cases = vec![
        ("短消息", 2, 3, 5),
        ("这是一个较长的消息，包含更多的词汇和字符", 17, 3, 20),
        ("", 1, 3, 4), // 空消息
    ];
    
    for (message, expected_prompt, expected_completion, expected_total) in test_cases {
        let (response, token_usage) = llm_service.process_message(message, &[]).await.unwrap();
        
        assert!(response.contains("模拟LLM响应"));
        assert_eq!(token_usage.prompt_tokens, expected_prompt);
        assert_eq!(token_usage.completion_tokens, expected_completion);
        assert_eq!(token_usage.total_tokens, expected_total);
    }
}

#[tokio::test]
async fn test_token_usage_struct() {
    let token_usage = TokenUsage {
        prompt_tokens: 10,
        completion_tokens: 20,
        total_tokens: 30,
    };
    
    assert_eq!(token_usage.prompt_tokens, 10);
    assert_eq!(token_usage.completion_tokens, 20);
    assert_eq!(token_usage.total_tokens, 30);
    
    // 验证令牌数量的一致性
    assert_eq!(token_usage.prompt_tokens + token_usage.completion_tokens, token_usage.total_tokens);
}

#[tokio::test]
async fn test_multiple_concurrent_requests() {
    let llm_service = LLMService::new(true);
    
    // 并发发送多个请求
    let mut handles = vec![];
    
    for i in 0..5 {
        let service = llm_service.clone();
        let handle = tokio::spawn(async move {
            let message = format!("并发测试消息 {}", i);
            service.process_message(&message, &[]).await
        });
        handles.push(handle);
    }
    
    // 等待所有请求完成
    let results: Vec<_> = futures::future::join_all(handles).await;
    
    // 验证所有请求都成功完成
    for result in results {
        let inner_result = result.unwrap(); // 从JoinHandle中获取结果
        assert!(inner_result.is_ok()); // 验证LLM处理结果
        let (response, token_usage) = inner_result.unwrap();
        assert!(response.contains("模拟LLM响应"));
        assert!(token_usage.total_tokens > 0);
    }
}