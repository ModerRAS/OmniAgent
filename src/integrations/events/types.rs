//! 事件类型定义
//!
//! 该模块定义了事件驱动系统中使用的事件类型和相关结构。

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// 事件类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    /// 消息事件
    Message,
    
    /// 任务事件
    Task,
    
    /// 状态变更事件
    StateChange,
    
    /// 错误事件
    Error,
    
    /// 自定义事件
    Custom(String),
}

/// 事件结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// 事件ID
    pub id: Uuid,
    
    /// 事件类型
    pub event_type: EventType,
    
    /// 事件源
    pub source: String,
    
    /// 事件目标
    pub target: String,
    
    /// 事件数据
    pub data: serde_json::Value,
    
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    
    /// 元数据
    pub metadata: Option<serde_json::Value>,
}

impl Event {
    /// 创建新的事件
    ///
    /// # 参数
    /// * `event_type` - 事件类型
    /// * `source` - 事件源
    /// * `target` - 事件目标
    /// * `data` - 事件数据
    /// * `metadata` - 元数据
    ///
    /// # 返回值
    /// 返回新的事件实例
    pub fn new(
        event_type: EventType,
        source: String,
        target: String,
        data: serde_json::Value,
        metadata: Option<serde_json::Value>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            source,
            target,
            data,
            timestamp: Utc::now(),
            metadata,
        }
    }
}