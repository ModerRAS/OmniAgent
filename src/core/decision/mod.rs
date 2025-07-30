//! 决策引擎模块

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use tracing::info;

/// 决策类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionType {
    Route,      // 路由决策
    Execute,    // 执行决策
    Learn,      // 学习决策
    Optimize,   // 优化决策
}

/// 决策规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub condition: String,  // 规则条件（简化为字符串）
    pub action: String,     // 执行动作
    pub priority: u32,      // 优先级
    pub enabled: bool,      // 是否启用
}

/// 决策结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionResult {
    pub decision_type: DecisionType,
    pub rule_id: String,
    pub action: String,
    pub confidence: f32,
    pub reasoning: String,
}

/// 学习记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningRecord {
    pub id: String,
    pub decision_id: String,
    pub context: String,
    pub outcome: String,
    pub feedback: f32,  // 反馈分数 (-1.0 到 1.0)
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// 决策引擎
pub struct DecisionEngine {
    rules: Arc<RwLock<HashMap<String, DecisionRule>>>,
    learning_records: Arc<RwLock<Vec<LearningRecord>>>,
}

impl DecisionEngine {
    /// 创建新的决策引擎
    pub fn new() -> Self {
        Self {
            rules: Arc::new(RwLock::new(HashMap::new())),
            learning_records: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// 注册决策规则
    pub async fn register_rule(&self, rule: DecisionRule) -> Result<(), String> {
        let mut rules = self.rules.write().await;
        rules.insert(rule.id.clone(), rule);
        Ok(())
    }

    /// 基于上下文做出决策
    pub async fn make_decision(&self, context: &str, decision_type: DecisionType) -> DecisionResult {
        info!("🧠 基于上下文做出决策: {}", context);
        
        // 简化的决策逻辑 - 实际实现中应该更复杂
        let rules = self.rules.read().await;
        
        // 查找匹配的规则（简化实现）
        let mut matched_rules: Vec<&DecisionRule> = rules
            .values()
            .filter(|rule| rule.enabled && self.matches_context(rule, context))
            .collect();
        
        // 按优先级排序
        matched_rules.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        // 选择最高优先级的规则
        if let Some(rule) = matched_rules.first() {
            info!("✅ 匹配到规则: {}", rule.name);
            
            DecisionResult {
                decision_type,
                rule_id: rule.id.clone(),
                action: rule.action.clone(),
                confidence: 0.8, // 简化置信度
                reasoning: format!("匹配到规则 '{}': {}", rule.name, rule.description),
            }
        } else {
            // 默认决策
            info!("🔄 使用默认决策");
            
            DecisionResult {
                decision_type: decision_type.clone(),
                rule_id: "default".to_string(),
                action: self.get_default_action(&decision_type),
                confidence: 0.5,
                reasoning: "使用默认决策规则".to_string(),
            }
        }
    }

    /// 检查规则是否匹配上下文（简化实现）
    fn matches_context(&self, rule: &DecisionRule, context: &str) -> bool {
        // 简单的包含检查
        context.contains(&rule.condition) || rule.condition == "*"
    }

    /// 获取默认动作
    fn get_default_action(&self, decision_type: &DecisionType) -> String {
        match decision_type {
            DecisionType::Route => "route_to_local_llm".to_string(),
            DecisionType::Execute => "execute_default_action".to_string(),
            DecisionType::Learn => "record_for_learning".to_string(),
            DecisionType::Optimize => "optimize_performance".to_string(),
        }
    }

    /// 记录学习反馈
    pub async fn record_learning(&self, decision_id: &str, context: &str, outcome: &str, feedback: f32) {
        let record = LearningRecord {
            id: uuid::Uuid::new_v4().to_string(),
            decision_id: decision_id.to_string(),
            context: context.to_string(),
            outcome: outcome.to_string(),
            feedback,
            timestamp: chrono::Utc::now(),
        };
        
        let mut learning_records = self.learning_records.write().await;
        learning_records.push(record);
        info!("📝 记录学习反馈，ID: {}", decision_id);
    }

    /// 获取所有规则
    pub async fn get_all_rules(&self) -> Vec<DecisionRule> {
        let rules = self.rules.read().await;
        rules.values().cloned().collect()
    }

    /// 获取学习记录
    pub async fn get_learning_records(&self) -> Vec<LearningRecord> {
        let learning_records = self.learning_records.read().await;
        learning_records.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_decision_engine_creation() {
        let engine = DecisionEngine::new();
        assert_eq!(engine.get_all_rules().await.len(), 0);
        assert_eq!(engine.get_learning_records().await.len(), 0);
    }

    #[tokio::test]
    async fn test_rule_registration() {
        let engine = DecisionEngine::new();
        
        let rule = DecisionRule {
            id: "test_rule".to_string(),
            name: "测试规则".to_string(),
            description: "用于测试的规则".to_string(),
            condition: "*".to_string(),
            action: "test_action".to_string(),
            priority: 100,
            enabled: true,
        };
        
        assert!(engine.register_rule(rule).await.is_ok());
        assert_eq!(engine.get_all_rules().await.len(), 1);
    }

    #[tokio::test]
    async fn test_decision_making() {
        let engine = DecisionEngine::new();
        
        let rule = DecisionRule {
            id: "weather_rule".to_string(),
            name: "天气查询规则".to_string(),
            description: "处理天气查询请求".to_string(),
            condition: "天气".to_string(),
            action: "route_to_weather_agent".to_string(),
            priority: 100,
            enabled: true,
        };
        
        engine.register_rule(rule).await.unwrap();
        
        let decision = engine.make_decision("查询天气情况", DecisionType::Route).await;
        assert_eq!(decision.rule_id, "weather_rule");
        assert_eq!(decision.action, "route_to_weather_agent");
    }
}