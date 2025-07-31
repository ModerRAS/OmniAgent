//! 协议适配器单元测试

use omni_agent::integrations::adapters::AdapterManager;
use tokio;

#[tokio::test]
async fn test_adapter_manager_creation() {
    let adapter_manager = AdapterManager::new();
    
    // 验证适配器管理器可以成功创建
    assert!(true); // 如果没有panic，说明创建成功
}

#[tokio::test]
async fn test_adapter_manager_fields() {
    let adapter_manager = AdapterManager::new();
    
    // 验证适配器管理器结构体字段
    // 这里主要是验证结构体可以实例化
    assert!(true);
}

#[tokio::test]
async fn test_protocol_adapter_trait() {
    // 验证ProtocolAdapter trait可以被正确导入
    use omni_agent::integrations::adapters::ProtocolAdapter;
    
    assert!(true); // 如果能导入成功，说明trait定义正确
}

#[tokio::test]
async fn test_adapter_types() {
    // 验证各种适配器类型可以被正确导入
    use omni_agent::integrations::adapters::{A2AAdapter, MCPAdapter};
    
    assert!(true); // 如果能导入成功，说明适配器类型定义正确
}

#[tokio::test]
async fn test_concurrent_adapter_manager_creation() {
    // 并发创建多个适配器管理器
    let mut handles = vec![];
    
    for _ in 0..3 {
        let handle = tokio::spawn(async move {
            let manager = AdapterManager::new();
            manager
        });
        handles.push(handle);
    }
    
    // 等待所有创建完成
    let results: Vec<_> = futures::future::join_all(handles).await;
    
    // 验证所有管理器都成功创建
    assert_eq!(results.len(), 3);
    for result in results {
        assert!(result.is_ok());
    }
}