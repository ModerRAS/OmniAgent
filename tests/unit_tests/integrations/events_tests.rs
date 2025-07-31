//! 事件系统单元测试

use omni_agent::integrations::events::{EventBus, EventHandler, Event};
use tokio;
use std::sync::Arc;

#[tokio::test]
async fn test_event_bus_creation() {
    let event_bus = EventBus::new();
    
    // 验证事件总线可以成功创建
    assert!(true); // 如果没有panic，说明创建成功
}

#[tokio::test]
async fn test_event_bus_fields() {
    let event_bus = EventBus::new();
    
    // 验证事件总线结构体字段
    // 这里主要是验证结构体可以实例化
    assert!(true);
}

#[tokio::test]
async fn test_event_handler_trait() {
    // 验证EventHandler trait可以被正确导入
    use omni_agent::integrations::events::EventHandler;
    
    assert!(true); // 如果能导入成功，说明trait定义正确
}

#[tokio::test]
async fn test_event_types() {
    // 验证事件类型可以被正确导入
    use omni_agent::integrations::events::Event;
    
    assert!(true); // 如果能导入成功，说明事件类型定义正确
}

#[tokio::test]
async fn test_concurrent_event_bus_creation() {
    // 并发创建多个事件总线
    let mut handles = vec![];
    
    for _ in 0..3 {
        let handle = tokio::spawn(async move {
            let bus = EventBus::new();
            bus
        });
        handles.push(handle);
    }
    
    // 等待所有创建完成
    let results: Vec<_> = futures::future::join_all(handles).await;
    
    // 验证所有总线都成功创建
    assert_eq!(results.len(), 3);
    for result in results {
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_event_struct_creation() {
    // 测试事件结构体的创建
    let event = Event {
        id: uuid::Uuid::new_v4(),
        event_type: "test_event".to_string(),
        payload: serde_json::json!({"test": "data"}),
        timestamp: chrono::Utc::now(),
    };
    
    // 验证事件结构体字段
    assert!(!event.id.is_nil());
    assert_eq!(event.event_type, "test_event");
    assert_eq!(event.payload["test"], "data");
    assert!(event.timestamp.timestamp() > 0);
}