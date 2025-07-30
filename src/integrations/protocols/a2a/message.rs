//! A2A协议消息定义
//!
//! 该模块定义了A2A协议中使用的消息结构和内容类型。

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A2A消息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2AMessage {
    /// 消息ID
    pub id: Uuid,
    
    /// 发送者标识
    pub sender: String,
    
    /// 接收者标识
    pub recipient: String,
    
    /// 消息内容
    pub content: A2AMessageContent,
    
    /// 时间戳
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// 元数据
    pub metadata: Option<serde_json::Value>,
}

/// A2A消息内容类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum A2AMessageContent {
    /// 文本消息
    Text {
        text: String,
    },
    
    /// 请求消息
    Request {
        request_type: String,
        payload: serde_json::Value,
    },
    
    /// 响应消息
    Response {
        response_type: String,
        payload: serde_json::Value,
    },
    
    /// 错误消息
    Error {
        code: String,
        message: String,
    },
}

impl A2AMessage {
    /// 创建新的A2A消息
    ///
    /// # 参数
    /// * `sender` - 发送者标识
    /// * `recipient` - 接收者标识
    /// * `content` - 消息内容
    /// * `metadata` - 元数据
    ///
    /// # 返回值
    /// 返回新的A2A消息实例
    pub fn new(
        sender: String,
        recipient: String,
        content: A2AMessageContent,
        metadata: Option<serde_json::Value>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender,
            recipient,
            content,
            timestamp: chrono::Utc::now(),
            metadata,
        }
    }
}