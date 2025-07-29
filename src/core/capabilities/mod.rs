//! 能力管理器模块 - 简化版

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

/// 能力信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub enabled: bool,
}

/// 能力管理器
pub struct CapabilityManager {
    capabilities: Arc<RwLock<HashMap<String, Capability>>>,
}

impl CapabilityManager {
    pub fn new() -> Self {
        Self {
            capabilities: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 注册能力
    pub async fn register_capability(&self, capability: Capability) -> Result<(), String> {
        let mut capabilities = self.capabilities.write().await;
        capabilities.insert(capability.id.clone(), capability);
        Ok(())
    }

    /// 获取所有能力
    pub async fn get_all_capabilities(&self) -> Vec<Capability> {
        let capabilities = self.capabilities.read().await;
        capabilities.values().cloned().collect()
    }
}