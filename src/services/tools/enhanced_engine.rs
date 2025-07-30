//! 增强版工具执行引擎 - 简化实现

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use serde_json::Value;
use tokio::sync::{RwLock, Semaphore};
use tracing::info;
use uuid::Uuid;

use super::{Tool, ToolError};

/// 工具执行阶段
#[derive(Debug, Clone)]
pub enum ExecutionPhase {
    Validation,
    PermissionCheck,
    ConcurrencyMgmt,
    Execution,
    ResultValidation,
    CacheManagement,
    Cleanup,
}

/// 工具执行状态
#[derive(Debug, Clone)]
pub enum ExecutionStatus {
    Pending,
    Running(ExecutionPhase),
    Completed,
    Failed(String),
}

/// 执行上下文
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub user_id: String,
    pub session_id: String,
    pub permissions: Vec<String>,
    pub max_concurrent: usize,
    pub cache_ttl: Duration,
}

/// 执行结果
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub id: String,
    pub tool_name: String,
    pub status: ExecutionStatus,
    pub result: Option<Value>,
    pub error: Option<String>,
    pub execution_time: Duration,
}

/// 增强版工具执行引擎
pub struct EnhancedToolExecutionEngine {
    tools: Arc<RwLock<HashMap<String, Arc<dyn Tool>>>>,
    execution_cache: Arc<RwLock<HashMap<String, (Value, Instant)>>>,
    semaphore: Arc<Semaphore>,
    cache_ttl: Duration,
}

impl EnhancedToolExecutionEngine {
    /// 创建新的增强版工具执行引擎
    pub fn new(max_concurrent: usize, cache_ttl: Duration) -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
            execution_cache: Arc::new(RwLock::new(HashMap::new())),
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            cache_ttl,
        }
    }

    /// 注册工具
    pub async fn register_tool(&self, tool: Arc<dyn Tool>) -> Result<(), ToolError> {
        let mut tools = self.tools.write().await;
        let tool_name = tool.name().to_string();
        tools.insert(tool_name, tool.clone());
        info!("✅ 注册工具: {}", tool.name());
        Ok(())
    }

    /// 执行工具（8阶段生命周期）
    pub async fn execute_tool(
        &self,
        tool_name: &str,
        parameters: Value,
        context: ExecutionContext,
    ) -> Result<ExecutionResult, ToolError> {
        let execution_id = Uuid::new_v4().to_string();
        info!("🚀 开始执行工具: {} (ID: {})", tool_name, execution_id);

        let start_time = Instant::now();

        // 阶段1：输入验证
        self.validate_input(tool_name, &parameters)?;

        // 阶段2：权限检查
        self.check_permissions(tool_name, &context)?;

        // 阶段3：并发控制
        let _permit = self.semaphore.acquire().await
            .map_err(|_| ToolError::ConcurrencyLimitExceeded("无法获取并发许可".to_string()))?;

        // 阶段4：缓存检查
        let cache_key = format!("{}:{}", tool_name, parameters.to_string());
        if let Some(cached_result) = self.get_from_cache(&cache_key).await {
            return Ok(ExecutionResult {
                id: execution_id,
                tool_name: tool_name.to_string(),
                status: ExecutionStatus::Completed,
                result: Some(cached_result),
                error: None,
                execution_time: start_time.elapsed(),
            });
        }

        // 阶段5：实际执行
        let result = self.execute_actual_tool(tool_name, parameters).await?;

        // 阶段6：结果验证
        let validated_result = self.validate_result(&result)?;

        // 阶段7：缓存存储
        self.store_in_cache(cache_key, validated_result.clone()).await;

        // 阶段8：清理
        info!("✅ 执行完成，耗时: {:?}", start_time.elapsed());

        Ok(ExecutionResult {
            id: execution_id,
            tool_name: tool_name.to_string(),
            status: ExecutionStatus::Completed,
            result: Some(validated_result),
            error: None,
            execution_time: start_time.elapsed(),
        })
    }

    /// 输入验证
    fn validate_input(&self, tool_name: &str, parameters: &Value) -> Result<(), ToolError> {
        if tool_name.is_empty() {
            return Err(ToolError::ValidationFailed("工具名称不能为空".to_string()));
        }
        if !parameters.is_object() {
            return Err(ToolError::ValidationFailed("参数必须是对象".to_string()));
        }
        info!("✅ 输入验证通过");
        Ok(())
    }

    /// 权限检查
    fn check_permissions(&self, tool_name: &str, context: &ExecutionContext) -> Result<(), ToolError> {
        if tool_name.contains("admin") && !context.permissions.contains(&"admin".to_string()) {
            return Err(ToolError::PermissionDenied("需要管理员权限".to_string()));
        }
        info!("✅ 权限检查通过");
        Ok(())
    }

    /// 从缓存获取结果
    async fn get_from_cache(&self, key: &str) -> Option<Value> {
        let mut cache = self.execution_cache.write().await;
        
        if let Some((value, timestamp)) = cache.get(key) {
            if timestamp.elapsed() < self.cache_ttl {
                info!("✅ 缓存命中");
                return Some(value.clone());
            } else {
                cache.remove(key);
            }
        }
        
        None
    }

    /// 实际执行工具
    async fn execute_actual_tool(&self, tool_name: &str, parameters: Value) -> Result<Value, ToolError> {
        let tools = self.tools.read().await;
        let tool = tools.get(tool_name)
            .ok_or_else(|| ToolError::ToolNotFound(tool_name.to_string()))?
            .clone();
        
        tool.execute(parameters).await
    }

    /// 结果验证
    fn validate_result(&self, result: &Value) -> Result<Value, ToolError> {
        if result.is_null() {
            return Err(ToolError::ValidationFailed("结果不能为空".to_string()));
        }
        info!("✅ 结果验证通过");
        Ok(result.clone())
    }

    /// 存储到缓存
    async fn store_in_cache(&self, key: String, value: Value) {
        let mut cache = self.execution_cache.write().await;
        cache.insert(key, (value, Instant::now()));
        info!("✅ 结果已缓存");
    }

    /// 清理过期缓存
    pub async fn cleanup_expired_cache(&self) {
        let mut cache = self.execution_cache.write().await;
        cache.retain(|_, (_, timestamp)| timestamp.elapsed() < self.cache_ttl);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct MockTool;

    #[async_trait::async_trait]
    impl Tool for MockTool {
        fn name(&self) -> &str {
            "mock_tool"
        }

        fn description(&self) -> &str {
            "Mock tool for testing"
        }

        async fn execute(
            &self,
            parameters: Value,
        ) -> Result<Value, ToolError> {
            Ok(json!({"echo": parameters}))
        }
    }

    #[tokio::test]
    async fn test_enhanced_engine_creation() {
        let engine = EnhancedToolExecutionEngine::new(5, Duration::from_secs(300));
        assert!(engine.execute_tool("nonexistent", json!({}), ExecutionContext {
            user_id: "test".to_string(),
            session_id: "test".to_string(),
            permissions: vec![],
            max_concurrent: 5,
            cache_ttl: Duration::from_secs(300),
        }).await.is_err());
    }

    #[tokio::test]
    async fn test_tool_registration_and_execution() {
        let engine = EnhancedToolExecutionEngine::new(5, Duration::from_secs(300));
        let tool = Arc::new(MockTool);
        
        engine.register_tool(tool).await.unwrap();
        
        let context = ExecutionContext {
            user_id: "test_user".to_string(),
            session_id: "test_session".to_string(),
            permissions: vec!["user".to_string()],
            max_concurrent: 5,
            cache_ttl: Duration::from_secs(300),
        };

        let result = engine.execute_tool("mock_tool", json!({"test": "data"}), context).await;
        
        assert!(result.is_ok());
        let execution = result.unwrap();
        assert!(matches!(execution.status, ExecutionStatus::Completed));
        assert!(execution.result.is_some());
    }
}