//! 能力管理器单元测试

use omni_agent::core::capabilities::{CapabilityManager, Capability};
use tokio;

#[tokio::test]
async fn test_register_capability() {
    let capability_manager = CapabilityManager::new();
    
    let capability = Capability {
        id: "test_capability_1".to_string(),
        name: "测试能力1".to_string(),
        description: "用于测试的能力1".to_string(),
        category: "test".to_string(),
        enabled: true,
    };
    
    let result = capability_manager.register_capability(capability).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_capability() {
    let capability_manager = CapabilityManager::new();
    
    let capability = Capability {
        id: "test_capability_2".to_string(),
        name: "测试能力2".to_string(),
        description: "用于测试的能力2".to_string(),
        category: "test".to_string(),
        enabled: true,
    };
    
    capability_manager.register_capability(capability.clone()).await.unwrap();
    
    let retrieved_capability = capability_manager.get_capability("test_capability_2").await;
    assert!(retrieved_capability.is_some());
    
    let retrieved = retrieved_capability.unwrap();
    assert_eq!(retrieved.id, capability.id);
    assert_eq!(retrieved.name, capability.name);
    assert_eq!(retrieved.description, capability.description);
    assert_eq!(retrieved.category, capability.category);
    assert_eq!(retrieved.enabled, capability.enabled);
}

#[tokio::test]
async fn test_get_all_capabilities() {
    let capability_manager = CapabilityManager::new();
    
    // 注册多个能力
    for i in 1..=3 {
        let capability = Capability {
            id: format!("test_capability_{}", i),
            name: format!("测试能力{}", i),
            description: format!("用于测试的能力{}", i),
            category: "test".to_string(),
            enabled: true,
        };
        capability_manager.register_capability(capability).await.unwrap();
    }
    
    let capabilities = capability_manager.get_all_capabilities().await;
    assert_eq!(capabilities.len(), 3);
    
    // 验证所有能力都已注册
    let capability_ids: Vec<String> = capabilities.iter().map(|c| c.id.clone()).collect();
    assert!(capability_ids.contains(&"test_capability_1".to_string()));
    assert!(capability_ids.contains(&"test_capability_2".to_string()));
    assert!(capability_ids.contains(&"test_capability_3".to_string()));
}

#[tokio::test]
async fn test_capability_enabled_status() {
    let capability_manager = CapabilityManager::new();
    
    // 注册一个禁用的能力
    let disabled_capability = Capability {
        id: "disabled_capability".to_string(),
        name: "禁用能力".to_string(),
        description: "一个禁用的能力".to_string(),
        category: "test".to_string(),
        enabled: false,
    };
    
    capability_manager.register_capability(disabled_capability).await.unwrap();
    
    let retrieved_capability = capability_manager.get_capability("disabled_capability").await;
    assert!(retrieved_capability.is_some());
    assert_eq!(retrieved_capability.unwrap().enabled, false);
}

#[tokio::test]
async fn test_capability_fields_validation() {
    let capability_manager = CapabilityManager::new();
    
    let capability = Capability {
        id: "field_test_capability".to_string(),
        name: "字段测试能力".to_string(),
        description: "用于测试所有字段的完整能力".to_string(),
        category: "validation".to_string(),
        enabled: true,
    };
    
    capability_manager.register_capability(capability.clone()).await.unwrap();
    
    let retrieved = capability_manager.get_capability("field_test_capability").await.unwrap();
    
    // 验证所有字段都正确存储和检索
    assert_eq!(retrieved.id, "field_test_capability");
    assert_eq!(retrieved.name, "字段测试能力");
    assert_eq!(retrieved.description, "用于测试所有字段的完整能力");
    assert_eq!(retrieved.category, "validation");
    assert_eq!(retrieved.enabled, true);
}