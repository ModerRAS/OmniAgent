//! 工作流引擎模块

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::core::orchestration::OrchestrationEngine;

/// 工作流状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

/// 工作流步骤
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,
    pub name: String,
    pub description: String,
    pub task_id: Option<String>,
    pub dependencies: Vec<String>,
    pub condition: Option<String>, // 执行条件
}

/// 工作流定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,
    pub status: WorkflowStatus,
    pub results: HashMap<String, String>, // 步骤结果存储
}

/// 工作流引擎
pub struct WorkflowEngine {
    orchestration_engine: Arc<OrchestrationEngine>,
    workflows: Arc<RwLock<HashMap<String, Workflow>>>,
}

impl WorkflowEngine {
    /// 创建新的工作流引擎
    pub fn new(orchestration_engine: Arc<OrchestrationEngine>) -> Self {
        Self {
            orchestration_engine,
            workflows: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 注册工作流
    pub async fn register_workflow(&self, workflow: Workflow) -> Result<(), String> {
        let mut workflows = self.workflows.write().await;
        workflows.insert(workflow.id.clone(), workflow);
        Ok(())
    }

    /// 执行工作流
    pub async fn execute_workflow(&self, workflow_id: &str, initial_input: &str) -> Result<String, String> {
        info!("🔄 开始执行工作流: {}", workflow_id);
        
        // 获取工作流定义
        let _workflow = {
            let workflows = self.workflows.read().await;
            workflows.get(workflow_id).cloned().ok_or("工作流未找到")?
        };
        
        // 更新工作流状态为运行中
        {
            let mut workflows = self.workflows.write().await;
            if let Some(wf) = workflows.get_mut(workflow_id) {
                wf.status = WorkflowStatus::Running;
            }
        }
        
        // 执行工作流步骤
        let final_result = self.execute_workflow_steps(workflow_id, initial_input).await?;
        
        // 更新工作流状态为完成
        {
            let mut workflows = self.workflows.write().await;
            if let Some(wf) = workflows.get_mut(workflow_id) {
                wf.status = WorkflowStatus::Completed;
            }
        }
        
        Ok(final_result)
    }

    /// 执行工作流步骤
    async fn execute_workflow_steps(&self, workflow_id: &str, initial_input: &str) -> Result<String, String> {
        let workflows = self.workflows.read().await;
        let workflow = workflows.get(workflow_id).ok_or("工作流未找到")?;
        
        let mut current_input = initial_input.to_string();
        let mut step_results = HashMap::new();
        
        // 按顺序执行步骤（简化实现，实际应该考虑依赖关系）
        for step in &workflow.steps {
            info!("⏭️  执行步骤: {}", step.name);
            
            // 执行编排任务
            let result = self.orchestration_engine.orchestrate(&current_input).await?;
            
            // 保存步骤结果
            step_results.insert(step.id.clone(), result.clone());
            current_input = result;
        }
        
        // 保存工作流结果
        {
            let mut workflows = self.workflows.write().await;
            if let Some(wf) = workflows.get_mut(workflow_id) {
                wf.results = step_results;
            }
        }
        
        Ok(current_input)
    }

    /// 获取工作流状态
    pub async fn get_workflow_status(&self, workflow_id: &str) -> Option<WorkflowStatus> {
        let workflows = self.workflows.read().await;
        workflows.get(workflow_id).map(|wf| wf.status.clone())
    }

    /// 获取所有工作流
    pub async fn get_all_workflows(&self) -> Vec<Workflow> {
        let workflows = self.workflows.read().await;
        workflows.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        router::IntelligentRouter,
        state::StateManager,
        capabilities::CapabilityManager,
    };

    #[tokio::test]
    async fn test_workflow_engine_creation() {
        let router = Arc::new(IntelligentRouter::new());
        let state_manager = Arc::new(StateManager::new());
        let capability_manager = Arc::new(CapabilityManager::new());
        let orchestration_engine = Arc::new(OrchestrationEngine::new(router, state_manager, capability_manager));
        
        let engine = WorkflowEngine::new(orchestration_engine);
        assert_eq!(engine.get_all_workflows().await.len(), 0);
    }

    #[tokio::test]
    async fn test_workflow_registration() {
        let router = Arc::new(IntelligentRouter::new());
        let state_manager = Arc::new(StateManager::new());
        let capability_manager = Arc::new(CapabilityManager::new());
        let orchestration_engine = Arc::new(OrchestrationEngine::new(router, state_manager, capability_manager));
        
        let engine = WorkflowEngine::new(orchestration_engine);
        
        let workflow = Workflow {
            id: "test_workflow".to_string(),
            name: "测试工作流".to_string(),
            description: "用于测试的工作流".to_string(),
            steps: vec![],
            status: WorkflowStatus::Pending,
            results: HashMap::new(),
        };
        
        assert!(engine.register_workflow(workflow).await.is_ok());
        assert_eq!(engine.get_all_workflows().await.len(), 1);
    }
}