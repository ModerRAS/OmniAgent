//! 工具执行引擎模块 - 简化版

use std::collections::HashMap;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 工具错误
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("工具执行失败: {0}")]
    ExecutionFailed(String),
    #[error("工具未找到: {0}")]
    ToolNotFound(String),
}

/// 工具 trait
#[async_trait::async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self, parameters: Value) -> Result<Value, ToolError>;
}

/// 工具执行引擎
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