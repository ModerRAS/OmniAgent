//! 工具执行引擎模块 - 实现8阶段生命周期

pub mod enhanced_engine;

use std::collections::HashMap;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

pub use enhanced_engine::*;

/// 工具错误
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("工具执行失败: {0}")]
    ExecutionFailed(String),
    #[error("工具未找到: {0}")]
    ToolNotFound(String),
    #[error("验证失败: {0}")]
    ValidationFailed(String),
    #[error("权限被拒绝: {0}")]
    PermissionDenied(String),
    #[error("并发限制超出: {0}")]
    ConcurrencyLimitExceeded(String),
    #[error("缓存错误: {0}")]
    CacheError(String),
}

/// 工具 trait
#[async_trait::async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self, parameters: Value) -> Result<Value, ToolError>;
}

/// 简化版工具执行引擎（向后兼容）
pub struct ToolExecutionEngine {
    tools: Arc<RwLock<HashMap<String, Arc<dyn Tool>>>>,
}

impl ToolExecutionEngine {
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 注册工具
    pub async fn register_tool(&self, tool: Arc<dyn Tool>) -> Result<(), ToolError> {
        let mut tools = self.tools.write().await;
        tools.insert(tool.name().to_string(), tool);
        Ok(())
    }

    /// 执行工具
    pub async fn execute_tool(
        &self,
        tool_name: &str,
        parameters: Value,
    ) -> Result<Value, ToolError> {
        let tools = self.tools.read().await;
        let tool = tools.get(tool_name)
            .ok_or_else(|| ToolError::ToolNotFound(tool_name.to_string()))?
            .clone();
        
        tool.execute(parameters).await
    }
}