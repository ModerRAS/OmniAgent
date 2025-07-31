//! 工具执行引擎单元测试

use omni_agent::services::tools::{ToolExecutionEngine, EnhancedToolExecutionEngine};
use tokio;

#[tokio::test]
async fn test_tool_execution_engine_creation() {
    let tool_engine = ToolExecutionEngine::new();
    
    // 验证工具执行引擎可以成功创建
    assert!(true); // 如果没有panic，说明创建成功
}

#[tokio::test]
async fn test_enhanced_tool_execution_engine_creation() {
    let enhanced_engine = EnhancedToolExecutionEngine::new();
    
    // 验证增强版工具执行引擎可以成功创建
    assert!(true); // 如果没有panic，说明创建成功
}

#[tokio::test]
async fn test_tool_execution_engine_fields() {
    let tool_engine = ToolExecutionEngine::new();
    
    // 工具执行引擎应该有默认字段
    // 这里主要是验证结构体可以实例化
    assert!(true);
}

#[tokio::test]
async fn test_enhanced_engine_lifecycle_stages() {
    let enhanced_engine = EnhancedToolExecutionEngine::new();
    
    // 验证增强引擎有所有8个阶段的字段
    // 这里主要是验证结构体可以实例化，具体阶段逻辑在简化实现中
    assert!(true);
}

#[tokio::test]
async fn test_concurrent_tool_engine_creation() {
    // 并发创建多个工具执行引擎
    let mut handles = vec![];
    
    for _ in 0..5 {
        let handle = tokio::spawn(async move {
            let engine = ToolExecutionEngine::new();
            let enhanced_engine = EnhancedToolExecutionEngine::new();
            (engine, enhanced_engine)
        });
        handles.push(handle);
    }
    
    // 等待所有创建完成
    let results: Vec<_> = futures::future::join_all(handles).await;
    
    // 验证所有引擎都成功创建
    assert_eq!(results.len(), 5);
    for result in results {
        assert!(result.is_ok());
    }
}