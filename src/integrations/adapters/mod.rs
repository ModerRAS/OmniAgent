//! 协议适配器模块 - 简化版

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 协议适配器错误
#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    #[error("模拟错误: {0}")]
    MockError(String),
}

/// 协议适配器 trait
#[async_trait::async_trait]
pub trait ProtocolAdapter: Send + Sync {
    async fn send_request(&self, request: &str) -> Result<String, AdapterError>;
    fn get_capabilities(&self) -> Vec<String>;
    fn get_name(&self) -> &str;
}

/// 适配器管理器
pub struct AdapterManager {
    adapters: Arc<RwLock<HashMap<String, Arc<dyn ProtocolAdapter>>>>,
}

impl AdapterManager {
    pub fn new() -> Self {
        Self {
            adapters: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 注册适配器
    pub async fn register_adapter(&self, adapter: Arc<dyn ProtocolAdapter>) -> Result<(), AdapterError> {
        let mut adapters = self.adapters.write().await;
        adapters.insert(adapter.get_name().to_string(), adapter);
        Ok(())
    }

    /// 根据能力选择适配器
    pub async fn select_adapter_by_capability(&self, capability: &str) -> Option<Arc<dyn ProtocolAdapter>> {
        let adapters = self.adapters.read().await;
        for adapter in adapters.values() {
            if adapter.get_capabilities().contains(&capability.to_string()) {
                return Some(adapter.clone());
            }
        }
        None
    }
}