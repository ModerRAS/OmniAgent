//! 数据库模型定义
//!
//! 该模块定义了数据库中使用的数据模型。

use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use chrono::{DateTime, Utc};

/// 智能体记录模型
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AgentRecord {
    /// 记录ID
    pub id: Uuid,
    
    /// 智能体名称
    pub name: String,
    
    /// 智能体类型
    pub agent_type: String,
    
    /// 智能体配置
    pub config: serde_json::Value,
    
    /// 创建时间
    pub created_at: DateTime<Utc>,
    
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 对话记录模型
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ConversationRecord {
    /// 记录ID
    pub id: Uuid,
    
    /// 对话标题
    pub title: String,
    
    /// 参与者列表
    pub participants: Vec<String>,
    
    /// 创建时间
    pub created_at: DateTime<Utc>,
    
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 消息记录模型
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MessageRecord {
    /// 记录ID
    pub id: Uuid,
    
    /// 对话ID
    pub conversation_id: Uuid,
    
    /// 发送者
    pub sender: String,
    
    /// 接收者
    pub recipient: String,
    
    /// 消息内容
    pub content: serde_json::Value,
    
    /// 消息类型
    pub message_type: String,
    
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    
    /// 元数据
    pub metadata: Option<serde_json::Value>,
}