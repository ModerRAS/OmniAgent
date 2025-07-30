//! 智能体编排引擎模块

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::core::{
    router::{IntelligentRouter, RouteTarget},
    state::StateManager,
    capabilities::CapabilityManager,
};

/// 编排任务状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrchestrationTaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

/// 编排任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationTask {
    pub id: String,
    pub name: String,
    pub description: String,
    pub target: RouteTarget,
    pub status: OrchestrationTaskStatus,
    pub dependencies: Vec<String>,
    pub result: Option<String>,
}

/// 智能体编排引擎
pub struct OrchestrationEngine {
    router: Arc<IntelligentRouter>,
    state_manager: Arc<StateManager>,
    capability_manager: Arc<CapabilityManager>,
    tasks: Arc<RwLock<HashMap<String, OrchestrationTask>>>,
}

impl OrchestrationEngine {
    /// 创建新的编排引擎
    pub fn new(
        router: Arc<IntelligentRouter>,
        state_manager: Arc<StateManager>,
        capability_manager: Arc<CapabilityManager>,
    ) -> Self {
        Self {
            router,
            state_manager,
            capability_manager,
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 编排智能体执行任务
    pub async fn orchestrate(&self, message: &str) -> Result<String, String> {
        info!("🤖 开始编排任务: {}", message);
        
        // 1. 使用路由器分析消息并决定处理方式
        let decision = self.router.route_message(message).await;
        info!("🧭 路由决策: {:?}", decision.target);
        
        // 2. 创建编排任务
        let task_id = uuid::Uuid::new_v4().to_string();
        let task = OrchestrationTask {
            id: task_id.clone(),
            name: format!("任务: {}", message),
            description: decision.reasoning.clone(),
            target: decision.target.clone(),
            status: OrchestrationTaskStatus::Pending,
            dependencies: vec![],
            result: None,
        };
        
        // 3. 注册任务
        {
            let mut tasks = self.tasks.write().await;
            tasks.insert(task_id.clone(), task);
        }
        
        // 4. 执行任务
        let result = self.execute_task(&task_id, message).await?;
        
        Ok(result)
    }

    /// 执行特定任务
    async fn execute_task(&self, task_id: &str, message: &str) -> Result<String, String> {
        // 更新任务状态为运行中
        {
            let mut tasks = self.tasks.write().await;
            if let Some(task) = tasks.get_mut(task_id) {
                task.status = OrchestrationTaskStatus::Running;
            }
        }
        
        // 获取任务目标并执行
        let result = {
            let tasks = self.tasks.read().await;
            if let Some(task) = tasks.get(task_id) {
                match &task.target {
                    RouteTarget::LocalLLM => {
                        // 这里应该调用LLM服务，但暂时返回模拟响应
                        format!("LLM处理结果: {}", message)
                    },
                    RouteTarget::A2AAgent(agent_name) => {
                        format!("A2A智能体 {} 处理结果", agent_name)
                    },
                    RouteTarget::MCPTool(tool_name) => {
                        format!("MCP工具 {} 执行结果", tool_name)
                    },
                }
            } else {
                return Err("任务未找到".to_string());
            }
        };
        
        // 更新任务状态为完成并保存结果
        {
            let mut tasks = self.tasks.write().await;
            if let Some(task) = tasks.get_mut(task_id) {
                task.status = OrchestrationTaskStatus::Completed;
                task.result = Some(result.clone());
            }
        }
        
        Ok(result)
    }

    /// 获取所有任务
    pub async fn get_all_tasks(&self) -> Vec<OrchestrationTask> {
        let tasks = self.tasks.read().await;
        tasks.values().cloned().collect()
    }

    /// 获取任务状态
    pub async fn get_task_status(&self, task_id: &str) -> Option<OrchestrationTaskStatus> {
        let tasks = self.tasks.read().await;
        tasks.get(task_id).map(|task| task.status.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_orchestration_engine_creation() {
        let router = Arc::new(IntelligentRouter::new());
        let state_manager = Arc::new(StateManager::new());
        let capability_manager = Arc::new(CapabilityManager::new());
        
        let engine = OrchestrationEngine::new(router, state_manager, capability_manager);
        assert_eq!(engine.get_all_tasks().await.len(), 0);
    }

    #[tokio::test]
    async fn test_task_execution() {
        let router = Arc::new(IntelligentRouter::new());
        let state_manager = Arc::new(StateManager::new());
        let capability_manager = Arc::new(CapabilityManager::new());
        
        let engine = OrchestrationEngine::new(router, state_manager, capability_manager);
        let result = engine.orchestrate("测试消息").await;
        
        assert!(result.is_ok());
        assert_eq!(engine.get_all_tasks().await.len(), 1);
    }
}